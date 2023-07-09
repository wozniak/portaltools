#![windows_subsystem = "windows"]
mod gui;

use native_windows_gui as nwg;
use nwg::NativeUi;

use image::{DynamicImage, Rgba, GenericImageView, Pixel, Rgb};
use std::path::{Path, PathBuf};

fn multiply_image_to_vtf(base: DynamicImage, c: &Rgba<u8>) -> Vec<u8> {
    let result = image::DynamicImage::ImageRgba8(imageproc::map::map_colors(
        &base.into_luma_alpha8(),
        |px| {
            let mut color = c.map_without_alpha(|c| (c as f32 * (px.0[0] as f32 / 255.)) as u8);
            color.channels_mut()[3] = px.0[1];

            color
        }
    ));

    vtf::create(result, vtf::ImageFormat::Rgba8888).unwrap()
}

impl gui::PortalTools {
    fn apply(&self) {
        match self.apply_result() {
            Ok(()) => nwg::modal_info_message(&self.window, "Portal Tools", "Success!"),
            Err(s) => nwg::modal_info_message(&self.window, "Portal Tools", s.as_str()),
        };
    }

    fn apply_result(&self) -> Result<(), String> {
        let base = self.game_box.text();
        if !((PathBuf::from(format!("{}/portal/bin/client.dll", base))).exists()
            && (PathBuf::from(format!("{}/hl2.exe", base))).exists())
        {
            return Err("Invalid game directory.".to_string());
        }

        for c in &[self.blue_box.text(), self.orange_box.text(), self.carry_box.text(), self.gun_box.text()] {
            hex::decode(c).map_err(|e| e.to_string())?;
        }

        if self.steampipe() {
            // create steampipe custom folders :vomit:
            std::fs::create_dir_all(
                PathBuf::from(format!("{}/portal/custom/portal_tools/materials/models/weapons/v_models/v_portalgun", base))
            ).map_err(|e| e.to_string());

            std::fs::create_dir_all(
                PathBuf::from(format!("{}/portal/custom/portal_tools/materials/models/weapons/w_models/portalgun", base))
            ).map_err(|e| e.to_string());

            std::fs::create_dir_all(
                PathBuf::from(format!("{}/portal/custom/portal_tools/materials/models/portals", base))
            ).map_err(|e| e.to_string());

            std::fs::create_dir_all(
                PathBuf::from(format!("{}/portal/custom/portal_tools/materials/sprites", base))
            ).map_err(|e| e.to_string());

            std::fs::create_dir_all(
                PathBuf::from(format!("{}/portal/custom/portal_tools/particles", base))
            ).map_err(|e| e.to_string());
        }

        if self.crosshair_check.check_state() == nwg::CheckBoxState::Checked {
            self.apply_crosshair()?
        }
        if self.portals_check.check_state() == nwg::CheckBoxState::Checked {
            self.apply_portals()?
        }
        if self.particles_check.check_state() == nwg::CheckBoxState::Checked {
            self.apply_particles()?
        }
        if self.gun_check.check_state() == nwg::CheckBoxState::Checked {
            self.apply_gun()?
        }
        Ok(())
    }

    fn apply_gun(&self) -> Result<(), String> {
        let v_gun_trans = image::load_from_memory(&include_bytes!("assets/v_portalgun.png")[..]).unwrap();
        let w_gun_trans = image::load_from_memory(&include_bytes!("assets/w_portalgun.png")[..]).unwrap();

        let gun_hex = hex::decode(&self.gun_box.text()).unwrap();
        let gun_color = Rgb::<u8>::from_slice(&gun_hex[..]).to_rgba();

        // color viewmodel
        let v_gun = image::DynamicImage::new_rgba8(v_gun_trans.width(), v_gun_trans.height());
        let mut v_gun = image::DynamicImage::ImageRgba8(imageproc::map::map_colors(&v_gun, |_| gun_color));
        image::imageops::overlay(&mut v_gun, &v_gun_trans, 0, 0);

        // color world model
        let w_gun = image::DynamicImage::new_rgba8(w_gun_trans.width(), w_gun_trans.height());
        let mut w_gun = image::DynamicImage::ImageRgba8(imageproc::map::map_colors(&w_gun, |_| gun_color));
        image::imageops::overlay(&mut w_gun, &w_gun_trans, 0, 0);

        std::fs::write(
            format!("{}/materials/models/weapons/v_models/v_portalgun/v_portalgun.vtf", self.prefix()),
            vtf::create(v_gun, vtf::ImageFormat::Rgba8888).map_err(|e| e.to_string())?
        )
            .map_err(|e| e.to_string())?;

        std::fs::write(
            format!("{}/materials/models/weapons/w_models/portalgun/w_portalgun.vtf", self.prefix()),
            vtf::create(w_gun, vtf::ImageFormat::Rgba8888).map_err(|e| e.to_string())?
        )
            .map_err(|e| e.to_string())?;

        Ok(())
    }
    // change the portal colors
    fn apply_portals(&self) -> Result<(), String> {
        let dx8_grey =
            image::load_from_memory(
                &include_bytes!("assets/dx8.png")[..],
            ).unwrap();

        let dx9_grey =
            image::load_from_memory(
                &include_bytes!("assets/dx9.png")[..],
            ).unwrap();

        let strider_greybeam =
            image::load_from_memory(
                &include_bytes!("assets/strider_bluebeam.png")[..],
            ).unwrap();

        let greylight =
            image::load_from_memory(
                &include_bytes!("assets/colorlight.png")[..],
            ).unwrap();


        let blue_hex = hex::decode(&self.blue_box.text()).unwrap();
        let blue = Rgb::<u8>::from_slice(&blue_hex[..]).to_rgba();

        let orange_hex = hex::decode(&self.orange_box.text()).unwrap();
        let orange = Rgb::<u8>::from_slice(&orange_hex[..]).to_rgba();


        // do the thing
        fn x(p: String, i: &DynamicImage, c: &Rgba<u8>) -> Result<(), String> {
            std::fs::write(
                p,
                multiply_image_to_vtf(i.clone(), c),
            )
                .map_err(|e| e.to_string())
        }

        // dx9
        x(
            format!("{}/materials/models/portals/portal-blue-color.vtf", self.prefix()),
            &dx9_grey,
            &blue
        )?;

        x(
            format!("{}/materials/models/portals/portal-orange-color.vtf", self.prefix()),
            &dx9_grey,
            &orange
        )?;

        // dx8
        x(
            format!("{}/materials/models/portals/portal-blue-color-dx8.vtf", self.prefix()),
            &dx8_grey,
            &blue
        )?;

        x(
            format!("{}/materials/models/portals/portal-orange-color-dx8.vtf", self.prefix()),
            &dx8_grey,
            &orange
        )?;

        // sprites
        x(
            format!("{}/materials/sprites/strider_bluebeam.vtf", self.prefix()),
            &strider_greybeam,
            &blue,
        )?;

        x(
            format!("{}/materials/sprites/bluelight.vtf", self.prefix()),
            &greylight,
            &blue,
        )?;

        x(
            format!("{}/materials/sprites/orangelight.vtf", self.prefix()),
            &greylight,
            &orange,
        )?;

        Ok(())
    }

    // change the particle colors
    fn apply_particles(&self) -> Result<(), String> {
        let portal_projectile_df = include_bytes!("assets/portal_projectile.pcf");
        let portalgun_df = include_bytes!("assets/portalgun.pcf");
        let portals_df = include_bytes!("assets/portals.pcf");

        let mut portal_projectile = portal_projectile_df.to_vec();
        let mut portals = portals_df.to_vec();
        let mut portalgun = portalgun_df.to_vec();

        let blue_hex = hex::decode(&self.blue_box.text()).unwrap();
        let orange_hex = hex::decode(&self.orange_box.text()).unwrap();

        for c in portal_projectile_df.windows(3).enumerate() {
            if c.1 == &[0x8C, 0xFF, 0xDB] {
                // replace the bytes with the color
                portal_projectile[c.0] = blue_hex[0];
                portal_projectile[c.0 + 1] = blue_hex[1];
                portal_projectile[c.0 + 2] = blue_hex[2];
            } else if c.1 == &[0xE6, 0x61, 0x00] {
                portal_projectile[c.0] = orange_hex[0];
                portal_projectile[c.0 + 1] = orange_hex[1];
                portal_projectile[c.0 + 2] = orange_hex[2];
            }
        }

        for c in portals_df.windows(3).enumerate() {
            if c.1 == &[0x8C, 0xFF, 0xDB] {
                portals[c.0] = blue_hex[0];
                portals[c.0 + 1] = blue_hex[1];
                portals[c.0 + 2] = blue_hex[2];
            } else if c.1 == &[0xE6, 0x61, 0x00] {
                portals[c.0] = orange_hex[0];
                portals[c.0 + 1] = orange_hex[1];
                portals[c.0 + 2] = orange_hex[2];
            }
        }

        for c in portalgun_df.windows(3).enumerate() {
            if c.1 == &[0x8C, 0xFF, 0xDB] {
                portalgun[c.0] = blue_hex[0];
                portalgun[c.0 + 1] = blue_hex[1];
                portalgun[c.0 + 2] = blue_hex[2];
            } else if c.1 == &[0xE6, 0x61, 0x00] {
                portalgun[c.0] = orange_hex[0];
                portalgun[c.0 + 1] = orange_hex[1];
                portalgun[c.0 + 2] = orange_hex[2];
            }
        }

        std::fs::write(format!("{}/particles/portalgun.pcf", self.prefix()), portalgun).map_err(|e| e.to_string())?;
        std::fs::write(format!("{}/particles/portal_projectile.pcf", self.prefix()), portal_projectile).map_err(|e| e.to_string())?;
        std::fs::write(format!("{}/particles/portals.pcf", self.prefix()), portals).map_err(|e| e.to_string())?;
        Ok(())
    }

    // apply crosshair changes
    fn apply_crosshair(&self) -> Result<(), String> {
       nwg::modal_info_message(&self.window, "Portal Tools", "Crosshair no longer does anything. Please use https://mikes.software/sst instead."); 
       Ok(())
    }

    fn steampipe(&self) -> bool {
        Path::new(&format!("{}/portal/portal_pak_dir.vpk", &self.game_box.text()))
            .exists()
    }

    fn prefix(&self) -> String {
        if self.steampipe() {
            format!("{}/portal/custom/portal_tools/", &self.game_box.text())
        } else {
            format!("{}/portal/", &self.game_box.text())
        }
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let mut font = nwg::Font::default();

    nwg::Font::builder()
        .family("Segoe UI")
        .size(14)
        .build(&mut font);

    nwg::Font::set_global_default(Some(font));

    let _calc = gui::PortalTools::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
