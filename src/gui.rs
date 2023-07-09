use native_windows_derive::NwgUi;
use native_windows_gui::*;

#[derive(NwgUi, Default)]
pub struct PortalTools {
    // layout and window
    #[nwg_control(flags: "WINDOW|VISIBLE", size: (420, 200), title: "Portal Tools")]
    #[nwg_events( OnWindowClose: [PortalTools::close] )]
    pub window: Window,

    #[nwg_layout(parent: window, spacing: 2)]
    layout: GridLayout,

    // blue color
    #[nwg_control(text: "Blue")]
    #[nwg_layout_item(layout: layout, row: 0, col: 0)]
    _blue_label: Label,

    #[nwg_control(text: "40a0ff")]
    #[nwg_layout_item(layout: layout, row: 0, col: 1, col_span: 3)]
    pub blue_box: TextInput,

    #[nwg_layout_item(layout: layout, row: 0, col: 4)]
    #[nwg_control(text: "Pick")]
    #[nwg_events(OnButtonClick: [PortalTools::pick_blue])]
    _blue_button: Button,

    // orange color
    #[nwg_control(text: "Orange")]
    #[nwg_layout_item(layout: layout, row: 1, col: 0)]
    _orange_label: Label,

    #[nwg_control(parent: window, text: "ffa020")]
    #[nwg_layout_item(layout: layout, row: 1, col: 1, col_span: 3)]
    pub orange_box: TextInput,

    #[nwg_layout_item(layout: layout, row: 1, col: 4)]
    #[nwg_control(text: "Pick")]
    #[nwg_events(OnButtonClick: [PortalTools::pick_orange])]
    _orange_button: Button,

    // prop carry color
    #[nwg_control(text: "Carry")]
    #[nwg_layout_item(layout: layout, row: 2, col: 0)]
    _carry_label: Label,

    #[nwg_control(parent: window, text: "f2caa7")]
    #[nwg_layout_item(layout: layout, row: 2, col: 1, col_span: 3)]
    pub carry_box: TextInput,

    #[nwg_layout_item(layout: layout, row: 2, col: 4)]
    #[nwg_control(text: "Pick")]
    #[nwg_events(OnButtonClick: [PortalTools::pick_carry])]
    _carry_button: Button,

    // gun color
    #[nwg_control(text: "Portal Gun")]
    #[nwg_layout_item(layout: layout, row: 3, col: 0)]
    _gun_label: Label,

    #[nwg_control(parent: window, text: "ffffff")]
    #[nwg_layout_item(layout: layout, row: 3, col: 1, col_span: 3)]
    pub gun_box: TextInput,

    #[nwg_layout_item(layout: layout, row: 3, col: 4)]
    #[nwg_control(text: "Pick")]
    #[nwg_events(OnButtonClick: [PortalTools::pick_gun])]
    _gun_button: Button,

    // game dir
    #[nwg_control(text: "Game")]
    #[nwg_layout_item(layout: layout, row: 4, col: 0)]
    _game_label: Label,

    #[nwg_control(parent: window, text: "")]
    #[nwg_layout_item(layout: layout, row: 4, col: 1, col_span: 3)]
    pub game_box: TextInput,

    #[nwg_layout_item(layout: layout, row: 4, col: 4)]
    #[nwg_control(text: "Browse")]
    #[nwg_events(OnButtonClick: [PortalTools::pick_game])]
    _game_button: Button,

    // options and apply button
    #[nwg_layout_item(layout: layout, row: 5, col: 1)]
    #[nwg_control(text: "Crosshair")]
    pub crosshair_check: CheckBox,

    #[nwg_layout_item(layout: layout, row: 5, col: 2)]
    #[nwg_control(text: "Portals")]
    pub portals_check: CheckBox,

    #[nwg_layout_item(layout: layout, row: 5, col: 3)]
    #[nwg_control(text: "Particles")]
    pub particles_check: CheckBox,

    #[nwg_layout_item(layout: layout, row: 5, col: 4)]
    #[nwg_control(text: "Portal Gun")]
    pub gun_check: CheckBox,

    #[nwg_layout_item(layout: layout, row: 5, col: 0)]
    #[nwg_control(text: "Apply")]
    #[nwg_events(OnButtonClick: [PortalTools::apply])]
    apply_button: Button,

    #[nwg_resource]
    picker: ColorDialog,

    #[nwg_resource(action: FileDialogAction::OpenDirectory, multiselect: false)]
    browser: FileDialog,
}

impl PortalTools {
    fn close(&self) {
        crate::nwg::stop_thread_dispatch();
    }

    fn pick_blue(&self) {
        if self.picker.run(Some(&self.window)) {
            let c = self.picker.color();
            self.blue_box
                .set_text(&format!("{:02x}{:02x}{:02x}", c[0], c[1], c[2]));
        }
    }
    fn pick_orange(&self) {
        if self.picker.run(Some(&self.window)) {
            let c = self.picker.color();
            self.orange_box
                .set_text(&format!("{:02x}{:02x}{:02x}", c[0], c[1], c[2]));
        }
    }
    fn pick_carry(&self) {
        if self.picker.run(Some(&self.window)) {
            let c = self.picker.color();
            self.carry_box
                .set_text(&format!("{:02x}{:02x}{:02x}", c[0], c[1], c[2]));
        }
    }
    fn pick_gun(&self) {
        if self.picker.run(Some(&self.window)) {
            let c = self.picker.color();
            self.gun_box
                .set_text(&format!("{:02x}{:02x}{:02x}", c[0], c[1], c[2]));
        }
    }
    fn pick_game(&self) {
        if self.browser.run(Some(&self.window)) {
            let path = self.browser.get_selected_item().unwrap();
            self.game_box.set_text(path.to_str().unwrap());
        }
    }
}
