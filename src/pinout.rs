macro_rules! pinout {
    ($p:ident . rgb_led0) => ($p.P0_29); // pull down active high
    ($p:ident . rgb_led1) => ($p.P0_30); // pull down active high
    ($p:ident . rgb_led2) => ($p.P0_31); // pull down active high
    ($p:ident . ir_led0) => ($p.P0_04); // pull down active high
    ($p:ident . ir_led1) => ($p.P1_09); // pull down active high
    ($p:ident . pwr_btn) => ($p.P0_11); // pull up active low
    ($p:ident . sync0) => ($p.P0_09);
    ($p:ident . drivec1) => ($p.P0_15);
    ($p:ident . drivec0) => ($p.P0_17);
    ($p:ident . drivev) => ($p.P0_20);
    ($p:ident . pwm0_out0) => ($p.P0_04);
    ($p:ident . pwm0_out1) => ($p.P1_09);
}
