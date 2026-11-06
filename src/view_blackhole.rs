use macroquad::prelude::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::thread;

use crate::blackhole::{
    BlackHoleImage, CRITICAL_IMPACT_PARAMETER, DISK_INNER_RADIUS, DISK_OUTER_RADIUS,
    EVENT_HORIZON_RADIUS, IMAGE_HEIGHT, IMAGE_WIDTH, MASS_LENGTH, OBSERVER_RADIUS,
    PHOTON_SPHERE_RADIUS, render_image_cancellable,
};

pub struct BlackHoleView {
    image: Texture2D,
    inclination_degrees: f64,
    summary: crate::blackhole::ImageSummary,
    render_receiver: Option<Receiver<(f64, BlackHoleImage)>>,
    render_cancel: Option<Arc<AtomicBool>>,
}

impl Default for BlackHoleView {
    fn default() -> Self {
        Self::new(65.0)
    }
}

impl BlackHoleView {
    pub fn new(inclination_degrees: f64) -> Self {
        let empty_pixels = vec![0; usize::from(IMAGE_WIDTH) * usize::from(IMAGE_HEIGHT) * 4];
        let image = Texture2D::from_rgba8(IMAGE_WIDTH, IMAGE_HEIGHT, &empty_pixels);
        image.set_filter(FilterMode::Linear);
        let mut view = Self {
            image,
            inclination_degrees,
            summary: crate::blackhole::ImageSummary::default(),
            render_receiver: None,
            render_cancel: None,
        };
        view.request_render();
        view
    }

    pub fn adjust_inclination(&mut self, change_degrees: f64) {
        let inclination = (self.inclination_degrees + change_degrees).clamp(20.0, 85.0);
        if (inclination - self.inclination_degrees).abs() > f64::EPSILON {
            self.inclination_degrees = inclination;
            self.request_render();
        }
    }

    pub fn reset(&mut self) {
        self.inclination_degrees = 65.0;
        self.request_render();
    }

    fn request_render(&mut self) {
        if let Some(cancelled) = self.render_cancel.take() {
            cancelled.store(true, Ordering::Relaxed);
        }
        let inclination = self.inclination_degrees;
        let cancelled = Arc::new(AtomicBool::new(false));
        let worker_cancelled = Arc::clone(&cancelled);
        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || {
            let frame =
                render_image_cancellable(IMAGE_WIDTH, IMAGE_HEIGHT, inclination, &worker_cancelled);
            let _ = sender.send((inclination, frame));
        });
        self.render_receiver = Some(receiver);
        self.render_cancel = Some(cancelled);
    }

    fn receive_render(&mut self) {
        let Some(receiver) = self.render_receiver.take() else {
            return;
        };
        match receiver.try_recv() {
            Ok((inclination, frame)) if (inclination - self.inclination_degrees).abs() < 1e-9 => {
                self.image = Texture2D::from_rgba8(frame.width, frame.height, &frame.rgba);
                self.image.set_filter(FilterMode::Linear);
                self.summary = frame.summary;
                self.render_cancel = None;
            }
            Ok(_) | Err(TryRecvError::Disconnected) => self.render_cancel = None,
            Err(TryRecvError::Empty) => self.render_receiver = Some(receiver),
        }
    }

    pub fn draw(&mut self) {
        self.receive_render();
        clear_background(Color::from_rgba(2, 3, 9, 255));
        let source_ratio = f32::from(IMAGE_WIDTH) / f32::from(IMAGE_HEIGHT);
        let screen_ratio = screen_width() / screen_height().max(1.0);
        let (draw_width, draw_height) = if screen_ratio > source_ratio {
            (screen_height() * source_ratio, screen_height())
        } else {
            (screen_width(), screen_width() / source_ratio)
        };
        draw_texture_ex(
            &self.image,
            (screen_width() - draw_width) * 0.5,
            (screen_height() - draw_height) * 0.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(draw_width, draw_height)),
                ..Default::default()
            },
        );

        draw_rectangle(16.0, 14.0, 468.0, 156.0, Color::from_rgba(5, 9, 20, 220));
        draw_text(
            "SCHWARZSCHILD BLACK HOLE | RAY-TRACED VIEW",
            28.0,
            42.0,
            19.0,
            WHITE,
        );
        draw_text(
            format!(
                "HORIZON: {:.1}M | PHOTON SPHERE: {:.1}M | SHADOW b: {:.3}M",
                EVENT_HORIZON_RADIUS / MASS_LENGTH,
                PHOTON_SPHERE_RADIUS / MASS_LENGTH,
                CRITICAL_IMPACT_PARAMETER / MASS_LENGTH
            ),
            28.0,
            68.0,
            14.0,
            SKYBLUE,
        );
        draw_text(
            format!(
                "THIN DISK: {:.0}M–{:.0}M | STATIC CAMERA: {:.0}M | INCLINATION: {:.0}°",
                DISK_INNER_RADIUS / MASS_LENGTH,
                DISK_OUTER_RADIUS / MASS_LENGTH,
                OBSERVER_RADIUS / MASS_LENGTH,
                self.inclination_degrees
            ),
            28.0,
            92.0,
            14.0,
            GOLD,
        );
        draw_text(
            format!(
                "RAYS: {} captured | {} disk | {} escaped | {} near-critical{}",
                self.summary.captured_rays,
                self.summary.disk_rays,
                self.summary.escaped_rays,
                self.summary.critical_rays,
                if self.render_receiver.is_some() {
                    " | rendering angle..."
                } else {
                    ""
                }
            ),
            28.0,
            116.0,
            14.0,
            GRAY,
        );
        draw_text(
            "Schwarzschild lensing | Newtonian disk emissivity + GR photon shifts | M=GM/c²",
            28.0,
            140.0,
            13.0,
            GRAY,
        );
        draw_text(
            "5: black hole | 4: collapse | A/D or ←/→: viewing angle | R: reset | Esc: close",
            22.0,
            screen_height() - 18.0,
            15.0,
            WHITE,
        );
    }
}
