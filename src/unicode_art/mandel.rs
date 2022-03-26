use std::io::Write;

use image::DynamicImage;

use super::{error::UnicodeArtError, UnicodeArt, UnicodeArtOption};

pub struct MandelAsciiArtOption {}

pub struct MandelAsciiArt<'a> {
    _options: &'a MandelAsciiArtOption,
}

impl<'a> MandelAsciiArtOption {
    pub fn new() -> Self {
        MandelAsciiArtOption {}
    }
}

impl UnicodeArtOption for MandelAsciiArtOption {
    fn new_unicode_art<'a>(
        &'a self,
        _image: &'a DynamicImage,
    ) -> Result<Box<dyn UnicodeArt + 'a>, UnicodeArtError> {
        Ok(Box::new(MandelAsciiArt { _options: self }))
    }
}

impl<'a> UnicodeArt for MandelAsciiArt<'a> {
    /**
     * #include <stdio.h>
     * main(n)
     * {
     * 	float r, i, R, I, b;
     * 	for (i = -1; i < 1; i += .06, puts(""))
     * 		for (r = -2; I = i, (R = r) < 1; r += .03, putchar(n + 31))
     * 			for (n = 0; b = I * I, 26 > n++ && R * R + b < 4;
     * 			     I = 2 * R * I + i, R = R * R - b + r)
     * 				;
     * }
     * }
     */
    fn write_all(&self, writer: &mut dyn Write) -> Result<(), UnicodeArtError> {
        let mut n;
        for ti in (-100..100).step_by(6) {
            // from -1 to 1 (exclusive)
            let i = f64::from(ti) * 0.01;
            for tr in (-200..101).step_by(3) {
                // from -2 to 1 (exclusive)
                let r = f64::from(tr) * 0.01;
                let mut ii = i;
                let mut rr = r;
                n = 0;
                loop {
                    let b = ii * ii;
                    if n > 26 {
                        break;
                    }
                    n += 1;
                    if rr * rr + b >= 4.0 {
                        break;
                    }
                    ii = 2.0 * rr * ii + i;
                    rr = rr * rr - b + r;
                }
                write!(writer, "{}", char::from(n + 31))?;
            }
            writeln!(writer)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbImage;
    use pretty_assertions::assert_eq;
    use std::io::BufWriter;

    #[test]
    fn test_generate_madel() {
        let image = DynamicImage::ImageRgb8(RgbImage::new(1, 1));
        let art = MandelAsciiArtOption {}.new_unicode_art(&image).unwrap();
        let mut buf = BufWriter::new(Vec::new());
        let _ = art.write_all(&mut buf);
        let bytes = buf.into_inner().unwrap();
        let actual = String::from_utf8(bytes).unwrap();
        assert_eq!(
            r##########################"         !!!!!!!!"""""""""""""""""""""""""""##########$$$$%%&(.)(*2%$#######""""""""!!!!!!!!!!!!!!!!!
        !!!!!!!"""""""""""""""""""""""""""###########$$$$%%&'(*0+('&%$$#######""""""""!!!!!!!!!!!!!!!
       !!!!!!""""""""""""""""""""""""""############$$$$$%&(**-:::1('&%$$$#######""""""""!!!!!!!!!!!!!
      !!!!!""""""""""""""""""""""""""############$$$%%%&'(+:::::::02*&%$$$$$######""""""""!!!!!!!!!!!
      !!!"""""""""""""""""""""""""############$$%%%%%&&&'(4:::::::8:'&&%%%$$$$$####"""""""""!!!!!!!!!
     !!!""""""""""""""""""""""""##########$$$%&&'2''''(())+7::::::1*)(('&%%%%%'&$###"""""""""!!!!!!!!
    !!!"""""""""""""""""""""""#######$$$$$$%%&(-:0/+*,::2::::::::::::5:::('''(.+&%$##"""""""""!!!!!!!
   !!""""""""""""""""""""""#####$$$$$$$$$%%%&&(*3:::7:::::::::::::::::::::,::8:1)%$$##""""""""""!!!!!
   !""""""""""""""""""""####$$$$$$$$$$$%%%%&'()*.8::::::::::::::::::::::::::::56&%$$###""""""""""!!!!
  !!""""""""""""""""####$%%%$$$$$$$$%%%%%&'):8:5:::::::::::::::::::::::::::::0*(&%%$$##""""""""""!!!!
  !"""""""""""######$$%%(+'&&&&&&&&&&&&&&''),3:::::::::::::::::::::::::::::::::+(()%$###""""""""""!!!
 !"""""""#########$$$$%%)3*()(()4+(('''''(*9::::::::::::::::::::::::::::::::::::::*%$###"""""""""""!!
 !"""##########$$$$$$%%&'(*/:7.13::/:+*))*-:::::::::::::::::::::::::::::::::::::,(&%$####""""""""""!!
 ""##########$$$$$$$%&&&()+0:::::::::::2,,0:::::::::::::::::::::::::::::::::::::::&$$####"""""""""""!
 "#########$$$$$$$%(''((*0:::::::::::::::1::::::::::::::::::::::::::::::::::::::,'%$$#####""""""""""!
 ########$%%%%%%&&'(+.,..5::::::::::::::::::::::::::::::::::::::::::::::::::::::'%%$$#####""""""""""!
 $$$%%&&(&&'''''(,*+.:::::::::::::::::::::::::::::::::::::::::::::::::::::::::*'&%$$$#####""""""""""!
 $$&%%'):)('))((),,,9::::::::::::::::::::::::::::::::::::::::::::::::::::::::,('&%$$$#####""""""""""!
 ##$$$##$%%%%%%&&&'(*8181::::::::::::::::::::::::::::::::::::::::::::::::::::::*&%$$$#####""""""""""!
 "#########$$$$%%%&(+(()*.:::::::::::::::4:::::::::::::::::::::::::::::::::::::::&%$$#####""""""""""!
 ""##########$$$$$$$%&&'+*-2::::::::::::..4::::::::::::::::::::::::::::::::::::::/&$$####"""""""""""!
 """"##########$$$$$$%&&'(*2::4::::::0.**+-:::::::::::::::::::::::::::::::::::::,(&%$####"""""""""""!
 !"""""##########$$$$%%&'-3.-*)*-:+)8(((()*.:::::::::::::::::::::::::::::::::::::,'%$####""""""""""!!
  !"""""""""#######$$$%%'4''&&&')('&&&&&''(+/::::::::::::::::::::::::::::::::::-5+-%$###""""""""""!!!
  !"""""""""""""""####$%&%%%%%%$$$%%%%%&&&')::::::::::::::::::::::::::::::::::.('&%$$###""""""""""!!!
   !"""""""""""""""""""###$$$$$$$$$$$$%%%%%&(-*-1:::::::::::::::::::::::::::::/(&%$$###""""""""""!!!!
   !!"""""""""""""""""""""#####$$$$$$$$$%%%%&'(+::::::::::::::::::::::::::0::::,7%$$##""""""""""!!!!!
    !!"""""""""""""""""""""""#######$$$$$$%%%&*:::4:+-::::::::::::::::::.)):7)+,(%$##""""""""""!!!!!!
    !!!""""""""""""""""""""""""##########$$$%&:)2/)(((+,*+,/::::::/,+))5(&&&&&'+%$##""""""""""!!!!!!!
     !!!!"""""""""""""""""""""""""###########$$%%%%%&&&''),::::::::8('&&%%%%$$$$###"""""""""!!!!!!!!!
      !!!!""""""""""""""""""""""""""############$$$%%%%&'(+::::::::-(&%%$$$$$#####"""""""""!!!!!!!!!!
       !!!!!""""""""""""""""""""""""""############$$$$$%%)+2,/:::,**'%$$$$#######""""""""!!!!!!!!!!!!
        !!!!!!"""""""""""""""""""""""""""###########$$$$$%&&'),:,)'&%$$$#######""""""""!!!!!!!!!!!!!!
         !!!!!!!!""""""""""""""""""""""""""###########$$$$%&'(.,,-*%%$#######"""""""!!!!!!!!!!!!!!!!!
"##########################,
            actual
        );
    }
}
