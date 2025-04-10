use super::*;

#[test]
fn fatty_acid() -> PolarsResult<()> {
    let data_frame = FATTY_ACIDS
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().mass(None).alias("Mass")])
        .collect()?;
    let mass = data_frame["Mass"]
        .as_materialized_series()
        .round(4)?
        .f64()?
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        mass,
        [
            Some(88.0524),  // C4U0
            Some(102.0681), // C5U0
            Some(116.0837), // C6U0
            Some(130.0994), // C7U0
            Some(144.1150), // C8U0
            Some(158.1307), // C9U0
            Some(172.1463), // C10U0
            Some(186.1620), // C11U0
            Some(200.1776), // C12U0
            Some(214.1933), // C13U0
            Some(228.2089), // C14U0
            Some(242.2246), // C15U0
            Some(256.2402), // C16U0
            Some(254.2246), // C16U1DC9
            Some(254.2246), // C16U1DT9
            Some(270.2559), // C17U0
            Some(284.2715), // C18U0
            Some(282.2559), // C18U1DC9
            Some(282.2559), // C18U1DT9
            Some(280.2402), // C18U2DC9DC12
            Some(278.2246), // C18U3DC6DC9DC12
            Some(278.2246), // C18U3DC8DT10DC12
            Some(278.2246), // C18U3DC9DC12DC15
            Some(278.2246), // C18U3DC9DT11DT13
            Some(278.2246), // C18U3DT9DT11DC13
            Some(278.2246), // C18U3DT9DT11DT13
            Some(276.2089), // C18U4DC6DC9DC12DC15
            Some(298.2872), // C19U0
            Some(312.3028), // C20U0
            Some(310.2872), // C20U1DC9
            Some(310.2872), // C20U1DC11
            Some(308.2715), // C20U2DC11DC14
            Some(306.2559), // C20U3DC5DC8DC11
            Some(306.2559), // C20U3DC8DC11DC14
            Some(306.2559), // C20U3DC11DC14DC17
            Some(304.2402), // C20U4DC5DC8DC11DC14
            Some(304.2402), // C20U4DC8DC11DC14DC17
            Some(302.2246), // C20U5DC5DC8DC11DC14DC17
            Some(326.3185), // C21U0
            Some(340.3341), // C22U0
            Some(338.3185), // C22U1DC13
            Some(336.3028), // C22U2DC13DC16
            Some(334.2872), // C22U3DC5DC13DC16
            Some(332.2715), // C22U4DC7DC10DC13DC16
            Some(330.2559), // C22U5DC7DC10DC13DC16DC19
            Some(328.2402), // C22U6DC4DC7DC10DC13DC16DC19
            Some(354.3498), // C23U0
            Some(368.3654), // C24U0
            Some(366.3498), // C24U1DC15
            Some(364.3341), // C24U2DC15DC18
            Some(362.3185), // C24U3DC12DC15DC18
            Some(360.3028), // C24U4DC9DC12DC15DC18
            Some(358.2872), // C24U5DC6DC9DC12DC15DC18
            Some(356.2715), // C24U6DC6DC9DC12DC15DC18DC21
            Some(382.3811), // C25U0
            Some(396.3967), // C26U0
            Some(394.3811), // C26U1DC17
            Some(410.4124), // C27U0
            Some(424.4280), // C28U0
            Some(438.4437), // C29U0
            Some(452.4593), // C30U0
            Some(450.4437), // C30U1DC21
            Some(466.4750), // C31U0
            Some(480.4906), // C32U0
            Some(494.5063), // C33U0
            Some(508.5219), // C34U0
            Some(522.5376), // C35U0
            Some(536.5532), // C36U0
            None,           //
        ],
    );
    Ok(())
}

#[test]
fn rco() -> PolarsResult<()> {
    let data_frame = FATTY_ACIDS
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().rco().mass(None).alias("Mass")])
        .collect()?;
    let mass = data_frame["Mass"]
        .as_materialized_series()
        .round(4)?
        .f64()?
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        mass,
        [
            Some(71.0497),  // C4U0
            Some(85.0653),  // C5U0
            Some(99.0810),  // C6U0
            Some(113.0966), // C7U0
            Some(127.1123), // C8U0
            Some(141.1279), // C9U0
            Some(155.1436), // C10U0
            Some(169.1592), // C11U0
            Some(183.1749), // C12U0
            Some(197.1905), // C13U0
            Some(211.2062), // C14U0
            Some(225.2218), // C15U0
            Some(239.2375), // C16U0
            Some(237.2218), // C16U1DC9
            Some(237.2218), // C16U1DT9
            Some(253.2531), // C17U0
            Some(267.2688), // C18U0
            Some(265.2531), // C18U1DC9
            Some(265.2531), // C18U1DT9
            Some(263.2375), // C18U2DC9DC12
            Some(261.2218), // C18U3DC6DC9DC12
            Some(261.2218), // C18U3DC8DT10DC12
            Some(261.2218), // C18U3DC9DC12DC15
            Some(261.2218), // C18U3DC9DT11DT13
            Some(261.2218), // C18U3DT9DT11DC13
            Some(261.2218), // C18U3DT9DT11DT13
            Some(259.2062), // C18U4DC6DC9DC12DC15
            Some(281.2844), // C19U0
            Some(295.3001), // C20U0
            Some(293.2844), // C20U1DC9
            Some(293.2844), // C20U1DC11
            Some(291.2688), // C20U2DC11DC14
            Some(289.2531), // C20U3DC5DC8DC11
            Some(289.2531), // C20U3DC8DC11DC14
            Some(289.2531), // C20U3DC11DC14DC17
            Some(287.2375), // C20U4DC5DC8DC11DC14
            Some(287.2375), // C20U4DC8DC11DC14DC17
            Some(285.2218), // C20U5DC5DC8DC11DC14DC17
            Some(309.3157), // C21U0
            Some(323.3314), // C22U0
            Some(321.3157), // C22U1DC13
            Some(319.3001), // C22U2DC13DC16
            Some(317.2844), // C22U3DC5DC13DC16
            Some(315.2688), // C22U4DC7DC10DC13DC16
            Some(313.2531), // C22U5DC7DC10DC13DC16DC19
            Some(311.2375), // C22U6DC4DC7DC10DC13DC16DC19
            Some(337.3470), // C23U0
            Some(351.3627), // C24U0
            Some(349.3470), // C24U1DC15
            Some(347.3314), // C24U2DC15DC18
            Some(345.3157), // C24U3DC12DC15DC18
            Some(343.3001), // C24U4DC9DC12DC15DC18
            Some(341.2844), // C24U5DC6DC9DC12DC15DC18
            Some(339.2688), // C24U6DC6DC9DC12DC15DC18DC21
            Some(365.3783), // C25U0
            Some(379.3940), // C26U0
            Some(377.3783), // C26U1DC17
            Some(393.4096), // C27U0
            Some(407.4253), // C28U0
            Some(421.4409), // C29U0
            Some(435.4566), // C30U0
            Some(433.4409), // C30U1DC21
            Some(449.4722), // C31U0
            Some(463.4879), // C32U0
            Some(477.5035), // C33U0
            Some(491.5192), // C34U0
            Some(505.5348), // C35U0
            Some(519.5505), // C36U0
            None,           //
        ],
    );
    Ok(())
}

#[test]
fn rcoo() -> PolarsResult<()> {
    let data_frame = FATTY_ACIDS
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .rcoo()
            .mass(None)
            .alias("Mass")])
        .collect()?;
    let mass = data_frame["Mass"]
        .as_materialized_series()
        .round(4)?
        .f64()?
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        mass,
        [
            Some(87.0446),  // C4U0
            Some(101.0603), // C5U0
            Some(115.0759), // C6U0
            Some(129.0916), // C7U0
            Some(143.1072), // C8U0
            Some(157.1229), // C9U0
            Some(171.1385), // C10U0
            Some(185.1542), // C11U0
            Some(199.1698), // C12U0
            Some(213.1855), // C13U0
            Some(227.2011), // C14U0
            Some(241.2168), // C15U0
            Some(255.2324), // C16U0
            Some(253.2168), // C16U1DC9
            Some(253.2168), // C16U1DT9
            Some(269.2481), // C17U0
            Some(283.2637), // C18U0
            Some(281.2481), // C18U1DC9
            Some(281.2481), // C18U1DT9
            Some(279.2324), // C18U2DC9DC12
            Some(277.2168), // C18U3DC6DC9DC12
            Some(277.2168), // C18U3DC8DT10DC12
            Some(277.2168), // C18U3DC9DC12DC15
            Some(277.2168), // C18U3DC9DT11DT13
            Some(277.2168), // C18U3DT9DT11DC13
            Some(277.2168), // C18U3DT9DT11DT13
            Some(275.2011), // C18U4DC6DC9DC12DC15
            Some(297.2794), // C19U0
            Some(311.2950), // C20U0
            Some(309.2794), // C20U1DC9
            Some(309.2794), // C20U1DC11
            Some(307.2637), // C20U2DC11DC14
            Some(305.2481), // C20U3DC5DC8DC11
            Some(305.2481), // C20U3DC8DC11DC14
            Some(305.2481), // C20U3DC11DC14DC17
            Some(303.2324), // C20U4DC5DC8DC11DC14
            Some(303.2324), // C20U4DC8DC11DC14DC17
            Some(301.2168), // C20U5DC5DC8DC11DC14DC17
            Some(325.3107), // C21U0
            Some(339.3263), // C22U0
            Some(337.3107), // C22U1DC13
            Some(335.2950), // C22U2DC13DC16
            Some(333.2794), // C22U3DC5DC13DC16
            Some(331.2637), // C22U4DC7DC10DC13DC16
            Some(329.2481), // C22U5DC7DC10DC13DC16DC19
            Some(327.2324), // C22U6DC4DC7DC10DC13DC16DC19
            Some(353.3420), // C23U0
            Some(367.3576), // C24U0
            Some(365.3420), // C24U1DC15
            Some(363.3263), // C24U2DC15DC18
            Some(361.3107), // C24U3DC12DC15DC18
            Some(359.2950), // C24U4DC9DC12DC15DC18
            Some(357.2794), // C24U5DC6DC9DC12DC15DC18
            Some(355.2637), // C24U6DC6DC9DC12DC15DC18DC21
            Some(381.3733), // C25U0
            Some(395.3889), // C26U0
            Some(393.3733), // C26U1DC17
            Some(409.4046), // C27U0
            Some(423.4202), // C28U0
            Some(437.4359), // C29U0
            Some(451.4515), // C30U0
            Some(449.4359), // C30U1DC21
            Some(465.4672), // C31U0
            Some(479.4828), // C32U0
            Some(493.4985), // C33U0
            Some(507.5141), // C34U0
            Some(521.5298), // C35U0
            Some(535.5454), // C36U0
            None,           //
        ],
    );
    Ok(())
}

#[test]
fn rcooh() -> PolarsResult<()> {
    let data_frame = FATTY_ACIDS
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .rcooh()
            .mass(None)
            .alias("Mass")])
        .collect()?;
    let mass = data_frame["Mass"]
        .as_materialized_series()
        .round(4)?
        .f64()?
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        mass,
        [
            Some(88.0524),  // C4U0
            Some(102.0681), // C5U0
            Some(116.0837), // C6U0
            Some(130.0994), // C7U0
            Some(144.1150), // C8U0
            Some(158.1307), // C9U0
            Some(172.1463), // C10U0
            Some(186.1620), // C11U0
            Some(200.1776), // C12U0
            Some(214.1933), // C13U0
            Some(228.2089), // C14U0
            Some(242.2246), // C15U0
            Some(256.2402), // C16U0
            Some(254.2246), // C16U1DC9
            Some(254.2246), // C16U1DT9
            Some(270.2559), // C17U0
            Some(284.2715), // C18U0
            Some(282.2559), // C18U1DC9
            Some(282.2559), // C18U1DT9
            Some(280.2402), // C18U2DC9DC12
            Some(278.2246), // C18U3DC6DC9DC12
            Some(278.2246), // C18U3DC8DT10DC12
            Some(278.2246), // C18U3DC9DC12DC15
            Some(278.2246), // C18U3DC9DT11DT13
            Some(278.2246), // C18U3DT9DT11DC13
            Some(278.2246), // C18U3DT9DT11DT13
            Some(276.2089), // C18U4DC6DC9DC12DC15
            Some(298.2872), // C19U0
            Some(312.3028), // C20U0
            Some(310.2872), // C20U1DC9
            Some(310.2872), // C20U1DC11
            Some(308.2715), // C20U2DC11DC14
            Some(306.2559), // C20U3DC5DC8DC11
            Some(306.2559), // C20U3DC8DC11DC14
            Some(306.2559), // C20U3DC11DC14DC17
            Some(304.2402), // C20U4DC5DC8DC11DC14
            Some(304.2402), // C20U4DC8DC11DC14DC17
            Some(302.2246), // C20U5DC5DC8DC11DC14DC17
            Some(326.3185), // C21U0
            Some(340.3341), // C22U0
            Some(338.3185), // C22U1DC13
            Some(336.3028), // C22U2DC13DC16
            Some(334.2872), // C22U3DC5DC13DC16
            Some(332.2715), // C22U4DC7DC10DC13DC16
            Some(330.2559), // C22U5DC7DC10DC13DC16DC19
            Some(328.2402), // C22U6DC4DC7DC10DC13DC16DC19
            Some(354.3498), // C23U0
            Some(368.3654), // C24U0
            Some(366.3498), // C24U1DC15
            Some(364.3341), // C24U2DC15DC18
            Some(362.3185), // C24U3DC12DC15DC18
            Some(360.3028), // C24U4DC9DC12DC15DC18
            Some(358.2872), // C24U5DC6DC9DC12DC15DC18
            Some(356.2715), // C24U6DC6DC9DC12DC15DC18DC21
            Some(382.3811), // C25U0
            Some(396.3967), // C26U0
            Some(394.3811), // C26U1DC17
            Some(410.4124), // C27U0
            Some(424.4280), // C28U0
            Some(438.4437), // C29U0
            Some(452.4593), // C30U0
            Some(450.4437), // C30U1DC21
            Some(466.4750), // C31U0
            Some(480.4906), // C32U0
            Some(494.5063), // C33U0
            Some(508.5219), // C34U0
            Some(522.5376), // C35U0
            Some(536.5532), // C36U0
            None,           //
        ],
    );
    Ok(())
}

#[test]
fn rcooch3() -> PolarsResult<()> {
    let data_frame = FATTY_ACIDS
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .rcooch3()
            .mass(None)
            .alias("Mass")])
        .collect()?;
    let mass = data_frame["Mass"]
        .as_materialized_series()
        .round(4)?
        .f64()?
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        mass,
        [
            Some(102.0681), // C4U0
            Some(116.0837), // C5U0
            Some(130.0994), // C6U0
            Some(144.1150), // C7U0
            Some(158.1307), // C8U0
            Some(172.1463), // C9U0
            Some(186.1620), // C10U0
            Some(200.1776), // C11U0
            Some(214.1933), // C12U0
            Some(228.2089), // C13U0
            Some(242.2246), // C14U0
            Some(256.2402), // C15U0
            Some(270.2559), // C16U0
            Some(268.2402), // C16U1DC9
            Some(268.2402), // C16U1DT9
            Some(284.2715), // C17U0
            Some(298.2872), // C18U0
            Some(296.2715), // C18U1DC9
            Some(296.2715), // C18U1DT9
            Some(294.2559), // C18U2DC9DC12
            Some(292.2402), // C18U3DC6DC9DC12
            Some(292.2402), // C18U3DC8DT10DC12
            Some(292.2402), // C18U3DC9DC12DC15
            Some(292.2402), // C18U3DC9DT11DT13
            Some(292.2402), // C18U3DT9DT11DC13
            Some(292.2402), // C18U3DT9DT11DT13
            Some(290.2246), // C18U4DC6DC9DC12DC15
            Some(312.3028), // C19U0
            Some(326.3185), // C20U0
            Some(324.3028), // C20U1DC9
            Some(324.3028), // C20U1DC11
            Some(322.2872), // C20U2DC11DC14
            Some(320.2715), // C20U3DC5DC8DC11
            Some(320.2715), // C20U3DC8DC11DC14
            Some(320.2715), // C20U3DC11DC14DC17
            Some(318.2559), // C20U4DC5DC8DC11DC14
            Some(318.2559), // C20U4DC8DC11DC14DC17
            Some(316.2402), // C20U5DC5DC8DC11DC14DC17
            Some(340.3341), // C21U0
            Some(354.3498), // C22U0
            Some(352.3341), // C22U1DC13
            Some(350.3185), // C22U2DC13DC16
            Some(348.3028), // C22U3DC5DC13DC16
            Some(346.2872), // C22U4DC7DC10DC13DC16
            Some(344.2715), // C22U5DC7DC10DC13DC16DC19
            Some(342.2559), // C22U6DC4DC7DC10DC13DC16DC19
            Some(368.3654), // C23U0
            Some(382.3811), // C24U0
            Some(380.3654), // C24U1DC15
            Some(378.3498), // C24U2DC15DC18
            Some(376.3341), // C24U3DC12DC15DC18
            Some(374.3185), // C24U4DC9DC12DC15DC18
            Some(372.3028), // C24U5DC6DC9DC12DC15DC18
            Some(370.2872), // C24U6DC6DC9DC12DC15DC18DC21
            Some(396.3967), // C25U0
            Some(410.4124), // C26U0
            Some(408.3967), // C26U1DC17
            Some(424.4280), // C27U0
            Some(438.4437), // C28U0
            Some(452.4593), // C29U0
            Some(466.4750), // C30U0
            Some(464.4593), // C30U1DC21
            Some(480.4906), // C31U0
            Some(494.5063), // C32U0
            Some(508.5219), // C33U0
            Some(522.5376), // C34U0
            Some(536.5532), // C35U0
            Some(550.5689), // C36U0
            None,           //
        ],
    );
    Ok(())
}
