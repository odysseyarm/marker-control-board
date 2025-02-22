macro_rules! pinout {
    ($p:ident . rgb_led0) => ($p.P0_29); // pull down active high
    ($p:ident . rgb_led1) => ($p.P0_30); // pull down active high
    ($p:ident . rgb_led2) => ($p.P0_31); // pull down active high
    ($p:ident . ir_led0) => ($p.P0_15); // pull down active high
    ($p:ident . ir_led1) => ($p.P1_09); // pull down active high
    ($p:ident . ir_led2) => ($p.P0_17); // pull down active high
    ($p:ident . ir_led3) => ($p.P0_20); // pull down active high
    ($p:ident . pwr_btn) => ($p.P0_11); // pull up active low
    ($pca:ident . drivev) => ($pca.io2);
    ($pca:ident . ir_iset1) => ($pca.io0);
    ($pca:ident . ir_iset0) => ($pca.io1);
    ($p:ident . scl) => ($p.P0_09);
    ($p:ident . sda) => ($p.P0_10);
}
