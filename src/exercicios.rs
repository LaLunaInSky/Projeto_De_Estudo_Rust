mod ex_001;
mod ex_002;
mod ex_003;
mod ex_004;
mod ex_005;
mod ex_006;
mod ex_007;
mod ex_008;
mod ex_009;
mod ex_010;
mod ex_011;
mod ex_012;
mod ex_013;
mod ex_014;
mod ex_015;
mod ex_016;
mod ex_017;
mod ex_018;
mod ex_019;
mod ex_020;
mod ex_021;
mod ex_022;
mod ex_023;
mod ex_024;
mod ex_025;
mod ex_026;
mod ex_027;
mod ex_028;
mod ex_029;
mod ex_030;

pub fn executar_o_exercício_x(número_do_exercício: u32, cabeçalho_do_programa: &String) {
    if número_do_exercício == 30 {
        ex_030::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 29 {
        ex_029::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 28 {
        ex_028::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 27 {
        ex_027::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 26 {
        ex_026::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 25 {
        ex_025::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 24 {
        ex_024::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 23 {
        ex_023::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 22 {
        ex_022::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 21 {
        ex_021::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 20 {
        ex_020::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 19 {
        ex_019::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 18 {
        ex_018::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 17 {
        ex_017::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 16 {
        ex_016::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 15 {
        ex_015::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 14 {
        ex_014::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 13 {
        ex_013::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 12 {
        ex_012::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 11 {
        ex_011::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 10 {
        ex_010::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 9 {
        ex_009::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 8 {
        ex_008::rodar_o_exercício(&cabeçalho_do_programa);
    } else if  número_do_exercício == 7 {
        ex_007::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 6 {
        ex_006::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 5 {
        ex_005::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 4 {
        ex_004::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 3 {
        ex_003::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 2 {
        ex_002::rodar_o_exercício(&cabeçalho_do_programa);
    } else {
        ex_001::rodar_o_exercício(&cabeçalho_do_programa);
    }
}