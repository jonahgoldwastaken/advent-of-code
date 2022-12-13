use anyhow::{anyhow, Result};
use aoc::Grid;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Point {
    point_type: PointType,
    x: usize,
    y: usize,
    elevation: usize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PointType {
    Start,
    End,
    Point,
}

fn main() -> Result<()> {
    let input = aoc::load_input("twelve")?;
    let grid: Grid<Point> = Grid::from(
        input
            .lines()
            .enumerate()
            .map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .map(|(j, c)| Point {
                        x: j,
                        y: i,
                        elevation: match c {
                            'S' => 0,
                            'E' => 'z' as usize - 'a' as usize,
                            x => x as usize - 'a' as usize,
                        },
                        point_type: match c {
                            'S' => PointType::Start,
                            'E' => PointType::End,
                            _ => PointType::Point,
                        },
                    })
                    .collect_vec()
            })
            .collect_vec(),
    );

    println!(
        "Part one: {}",
        calculate_shortest_route(grid.clone(), |p| p.point_type == PointType::Start)?
    );
    println!(
        "Part two: {}",
        calculate_shortest_route(grid, |p| p.elevation == 0)?
    );
    Ok(())
}

fn calculate_shortest_route<FN>(grid: Grid<Point>, pred: FN) -> Result<usize>
where
    FN: FnOnce(&Point) -> bool + Copy,
{
    let end = find_end(&grid);
    if end.is_none() {
        return Err(anyhow!("No end point found"));
    }
    let end = end.unwrap();
    let mut routes: Vec<(Point, usize)> = vec![(end, 0)];
    let mut calculation_completed = None;
    while calculation_completed.is_none() {
        for (point, steps) in routes.clone() {
            let new_routes = get_walkable_adjacent_points(&grid, point, steps, &routes);
            if !new_routes.is_empty() {
                routes.extend(new_routes);
            }
        }
        calculation_completed = routes.iter().find(|(p, _)| pred(p));
    }
    Ok(calculation_completed.unwrap().1)
}

fn find_end(grid: &Grid<Point>) -> Option<Point> {
    let mut point: Option<_> = None;
    for row in grid.rows() {
        for p in row {
            if p.point_type == PointType::End {
                point = Some(*p);
            }
        }
    }
    point
}

fn get_walkable_adjacent_points(
    grid: &Grid<Point>,
    point: Point,
    steps: usize,
    routes: &[(Point, usize)],
) -> Vec<(Point, usize)> {
    let mut new_points: Vec<_> = Vec::new();
    let row = grid.row(point.y).unwrap();
    let col = grid.col(point.x).unwrap();
    let elevation_range = point.elevation.saturating_sub(1)..='z' as usize;
    if point.x > 0 {
        new_points.push((row[point.x - 1], steps + 1));
    }
    if point.x < row.len() - 1 {
        new_points.push((row[point.x + 1], steps + 1));
    }
    if point.y > 0 {
        new_points.push((col[point.y - 1], steps + 1));
    }
    if point.y < col.len() - 1 {
        new_points.push((col[point.y + 1], steps + 1));
    }
    new_points
        .into_iter()
        .filter(|(point, _)| {
            !routes.iter().any(|(p, _)| *p == *point) && elevation_range.contains(&point.elevation)
        })
        .collect()
}
