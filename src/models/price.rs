use uuid::Uuid;

#[derive(Queryable)]
pub struct Price {
    pub price_id: Uuid,
    pub publication_id: Uuid,
    pub currencty_code: CurrencyCode,
    pub unit_price: f64,
}

#[derive(Debug, PartialEq, DbEnum)]
#[derive(juniper::GraphQLEnum)]
#[DieselType = "Currency_code"]
pub enum CurrencyCode {
  Adp,
  Aed,
  Afa,
  Afn,
  Alk,
  All,
  Amd,
  Ang,
  Aoa,
  Aok,
  Aon,
  Aor,
  Ara,
  Arp,
  Ars,
  Ary,
  Ats,
  Aud,
  Awg,
  Aym,
  Azm,
  Azn,
  Bad,
  Bam,
  Bbd,
  Bdt,
  Bec,
  Bef,
  Bel,
  Bgj,
  Bgk,
  Bgl,
  Bgn,
  Bhd,
  Bif,
  Bmd,
  Bnd,
  Bob,
  Bop,
  Bov,
  Brb,
  Brc,
  Bre,
  Brl,
  Brn,
  Brr,
  Bsd,
  Btn,
  Buk,
  Bwp,
  Byb,
  Byn,
  Byr,
  Bzd,
  Cad,
  Cdf,
  Chc,
  Che,
  Chf,
  Chw,
  Clf,
  Clp,
  Cny,
  Cop,
  Cou,
  Crc,
  Csd,
  Csj,
  Csk,
  Cuc,
  Cup,
  Cve,
  Cyp,
  Czk,
  Ddm,
  Dem,
  Djf,
  Dkk,
  Dop,
  Dzd,
  Ecs,
  Ecv,
  Eek,
  Egp,
  Ern,
  Esa,
  Esb,
  Esp,
  Etb,
  Eur,
  Fim,
  Fjd,
  Fkp,
  Frf,
  Gbp,
  Gek,
  Gel,
  Ghc,
  Ghp,
  Ghs,
  Gip,
  Gmd,
  Gne,
  Gnf,
  Gns,
  Gqe,
  Grd,
  Gtq,
  Gwe,
  Gwp,
  Gyd,
  Hkd,
  Hnl,
  Hrd,
  Hrk,
  Htg,
  Huf,
  Idr,
  Iep,
  Ilp,
  Ilr,
  Ils,
  Inr,
  Iqd,
  Irr,
  Isj,
  Isk,
  Itl,
  Jmd,
  Jod,
  Jpy,
  Kes,
  Kgs,
  Khr,
  Kmf,
  Kpw,
  Krw,
  Kwd,
  Kyd,
  Kzt,
  Laj,
  Lak,
  Lbp,
  Lkr,
  Lrd,
  Lsl,
  Lsm,
  Ltl,
  Ltt,
  Luc,
  Luf,
  Lul,
  Lvl,
  Lvr,
  Lyd,
  Mad,
  Mdl,
  Mga,
  Mgf,
  Mkd,
  Mlf,
  Mmk,
  Mnt,
  Mop,
  Mro,
  Mru,
  Mtl,
  Mtp,
  Mur,
  Mvq,
  Mvr,
  Mwk,
  Mxn,
  Mxp,
  Mxv,
  Myr,
  Mze,
  Mzm,
  Mzn,
  Nad,
  Ngn,
  Nic,
  Nio,
  Nlg,
  Nok,
  Npr,
  Nzd,
  Omr,
  Pab,
  Peh,
  Pei,
  Pen,
  Pes,
  Pgk,
  Php,
  Pkr,
  Pln,
  Plz,
  Pte,
  Pyg,
  Qar,
  Rhd,
  Rok,
  Rol,
  Ron,
  Rsd,
  Rub,
  Rur,
  Rwf,
  Sar,
  Sbd,
  Scr,
  Sdd,
  Sdg,
  Sdp,
  Sek,
  Sgd,
  Shp,
  Sit,
  Skk,
  Sll,
  Sos,
  Srd,
  Srg,
  Ssp,
  Std,
  Stn,
  Sur,
  Svc,
  Syp,
  Szl,
  Thb,
  Tjr,
  Tjs,
  Tmm,
  Tmt,
  Tnd,
  Top,
  Tpe,
  Trl,
  Try,
  Ttd,
  Twd,
  Tzs,
  Uah,
  Uak,
  Ugs,
  Ugw,
  Ugx,
  Usd,
  Usn,
  Uss,
  Uyi,
  Uyn,
  Uyp,
  Uyu,
  Uyw,
  Uzs,
  Veb,
  Vef,
  Ves,
  Vnc,
  Vnd,
  Vuv,
  Wst,
  Xaf,
  Xag,
  Xau,
  Xba,
  Xbb,
  Xbc,
  Xbd,
  Xcd,
  Xdr,
  Xeu,
  Xfo,
  Xfu,
  Xof,
  Xpd,
  Xpf,
  Xpt,
  Xre,
  Xsu,
  Xts,
  Xua,
  Xxx,
  Ydd,
  Yer,
  Yud,
  Yum,
  Yun,
  Zal,
  Zar,
  Zmk,
  Zmw,
  Zrn,
  Zrz,
  Zwc,
  Zwd,
  Zwl,
  Zwn,
  Zwr,
}