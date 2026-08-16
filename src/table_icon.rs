use crate::assets::Icon;
use arcdps::imgui::{Id, TableColumnFlags, TableColumnSetup, TableFlags, TableToken, Ui};
use windows::core::Interface;

/// Renders a table with (optional) icon headers.
///
/// Ported locally from `arc_util::ui::render::table` (rather than depended on) to keep the
/// `windows` crate version consistent with [`crate::assets::Icon`].
pub fn table_with_icons<'ui, N>(
    ui: &'ui Ui,
    label: impl AsRef<str>,
    columns: &[TableIconColumn<N>],
    flags: TableFlags,
    show_icons: bool,
) -> Option<TableToken<'ui>>
where
    N: AsRef<str>,
{
    table_with_icons_sizing(ui, label, columns, flags, show_icons, [0.0, 0.0], 0.0)
}

/// Renders a table with (optional) icon headers and sizing parameters.
pub fn table_with_icons_sizing<'ui, N>(
    ui: &'ui Ui,
    label: impl AsRef<str>,
    columns: &[TableIconColumn<N>],
    flags: TableFlags,
    show_icons: bool,
    outer_size: [f32; 2],
    inner_size: f32,
) -> Option<TableToken<'ui>>
where
    N: AsRef<str>,
{
    if let Some(token) =
        ui.begin_table_with_sizing(label, columns.len(), flags, outer_size, inner_size)
    {
        ui.table_setup_scroll_freeze(0, 1);
        for column in columns {
            ui.table_setup_column_with(column.as_setup());
        }

        if show_icons {
            for column in columns {
                table_header_icon(ui, column.name.as_ref(), column.icon);
            }
        } else {
            ui.table_headers_row();
        }

        Some(token)
    } else {
        None
    }
}

/// A table column setup with icon.
#[derive(Debug, Clone)]
pub struct TableIconColumn<'i, Name> {
    pub name: Name,
    pub icon: Option<&'i Icon>,
    pub flags: TableColumnFlags,
    pub init_width_or_weight: f32,
    pub user_id: Id,
}

impl<'i, Name> TableIconColumn<'i, Name> {
    /// Creates a new icon column.
    pub fn new(name: Name, icon: Option<&'i Icon>) -> Self {
        Self::with_flags(name, icon, Default::default())
    }

    /// Creates a new icon column with given flags.
    pub fn with_flags(name: Name, icon: Option<&'i Icon>, flags: TableColumnFlags) -> Self {
        Self::with_id(name, icon, flags, 0.0, Default::default())
    }

    /// Creates a new icon column with given width/weight and id.
    pub fn with_id(
        name: Name,
        icon: Option<&'i Icon>,
        flags: TableColumnFlags,
        init_width_or_weight: f32,
        user_id: Id,
    ) -> Self {
        Self {
            name,
            icon,
            flags,
            init_width_or_weight,
            user_id,
        }
    }

    /// Generates the equivalent [`TableColumnSetup`].
    pub fn as_setup(&self) -> TableColumnSetup<&str>
    where
        Name: AsRef<str>,
    {
        TableColumnSetup {
            name: self.name.as_ref(),
            flags: self.flags,
            init_width_or_weight: self.init_width_or_weight,
            user_id: self.user_id,
        }
    }
}

/// Renders a table header with icon.
fn table_header_icon(ui: &Ui, label: impl AsRef<str>, icon: Option<&Icon>) {
    let label = label.as_ref();
    ui.table_next_column();
    if let Some(icon) = icon {
        let size = ui.text_line_height_with_spacing();
        let [pos_x, pos_y] = ui.cursor_screen_pos();
        let top = pos_y + (ui.text_line_height() - size) / 2.0;

        ui.set_cursor_screen_pos([pos_x + size, pos_y]);
        ui.table_header(format!("##{label}"));

        let ptr = icon.as_raw();
        ui.get_window_draw_list()
            .add_image(ptr.into(), [pos_x, top], [pos_x + size, top + size])
            .build();
    } else {
        ui.table_header(label);
    }
}
