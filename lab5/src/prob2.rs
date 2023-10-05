struct Canvas([[char; 50]; 10]);

impl Canvas
{
    fn set_point(&mut self, x: u8, y: u8, val: char)
    {
        if (x as usize) < self.0.len() && (y as usize) < self.0[0].len()
        {
            self.0[x as usize][y as usize] = val;
        }
    }

    fn print_canvas(&self)
    {
        for line in self.0
        {
            for element in line
            {
                print!("{}", element);
            }
            println!("");
        }
    }
}

pub fn prob2_start()
{
    let mut canvas = Canvas([[' '; 50]; 10]);
    canvas.set_point(1, 1, 'M');
    canvas.set_point(2, 2, 'a');
    canvas.set_point(3, 3, 'l');
    canvas.set_point(4, 4, 'i');
    canvas.set_point(5, 5, 'n');
    canvas.set_point(6, 6, 'a');
    canvas.print_canvas();
}