const HEIGHT: i32 = 15; // Ширина (в діапазоні 10..=80)
const WIDTH: i32 = 30; // Висота (в діапазоні 10..=80)

fn generate_envelope_image(height: i32, width: i32) -> String
{
    let mut stt = String::from("");

    for x in 0..height{
        for y in 0..width{
            if x == 0 || x == height - 1 {//Верхня та нижня строка
                stt.push('*')
            }
            else if y == 0 || y == width - 1 {//Ліва та права сторона
                stt.push('*')
            }

            else if (y - 1) / 2 == x && y%2 == 1 && y<width/2 && x<height/2   {//верх-ліво
                stt.push('*')
            }
            else if (width - y - 1) / 2 == x && y%2 == 0 && y>width/2 && x<height/2     {//верх-право
                stt.push('*')
            }
            else if (height - x - 1)*2==y && y<width/2 && x>height/2 - 1    {//низ ліво
                //let h = (height - x)*2;
                //println!("{height}, {x}, {h}, {y}");
                stt.push('*')
            }
            else if (height - x) * 2 == (width - y + 1)   && y>width/2 - 1 && x>height/2 - 1     {//низ право
                stt.push('*')
            }

            else {
                stt.push(' ')
            }
        }
        stt.push('\n')
    }
    return stt
}

fn main() {
    let x = generate_envelope_image(HEIGHT, WIDTH);
    println!("{x}");
}
