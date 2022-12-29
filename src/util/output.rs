use term_grid::{Cell, Direction, Filling, Grid, GridOptions};
use terminal_size::terminal_size;

pub fn get_grid(stuff: Vec<String>, one_line: bool) -> String {
    let one_line_list = stuff.join("\n");

    if one_line {
        return one_line_list;
    }

    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(2),
        direction: Direction::TopToBottom,
    });

    grid.reserve(stuff.len());
    let mut max_width: usize = 0;

    for val in stuff {
        let cell = Cell::from(val);
        if cell.width > max_width {
            max_width = cell.width;
        }
        grid.add(cell);
    }

    let width: usize = match terminal_size() {
        Some((width, _)) => width.0 as usize,
        None => match std::env::var_os("COLUMNS") {
            Some(columns) => columns.to_str().and_then(|s| s.parse().ok()).unwrap_or(80),
            None => 80,
        },
    };

    match grid.fit_into_width(width) {
        Some(display) => display.to_string(),
        None => one_line_list,
    }
}
