//! Easing Functions



use std::f32::consts::PI;



#[inline]
fn powf(base: f32, exp: f32) -> f32 {
    base.powf(exp)
}

/// No easing, just `y = x`.
#[inline]
pub fn linear(t: f32) -> f32 {
    t
}

/// <https://easings.net/#easeInQuad>
///
/// Modeled after the parabola `y = x^2`.
#[inline]
pub fn quadratic_in(t: f32) -> f32 {
    t * t
}

/// <https://easings.net/#easeOutQuad>
///
/// Same as `1.0 - quadratic_in(1.0 - t)`.
#[inline]
pub fn quadratic_out(t: f32) -> f32 {
    -(t * (t - 2.0))
}

/// <https://easings.net/#easeInOutQuad>
#[inline]
pub fn quadratic_in_out(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        (-2.0 * t * t) + (4.0 * t) - 1.0
    }
}

/// <https://easings.net/#easeInCubic>
///
/// Modeled after the parabola `y = x^3`.
#[inline]
pub fn cubic_in(t: f32) -> f32 {
    t * t * t
}

/// <https://easings.net/#easeOutCubic>
#[inline]
pub fn cubic_out(t: f32) -> f32 {
    let f = t - 1.0;
    f * f * f + 1.0
}

/// <https://easings.net/#easeInOutCubic>
#[inline]
pub fn cubic_in_out(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        let f = (2.0 * t) - 2.0;
        0.5 * f * f * f + 1.0
    }
}

/// <https://easings.net/#easeInSine>
///
/// Modeled after quarter-cycle of sine wave.
#[inline]
pub fn sin_in(t: f32) -> f32 {
    ((t - 1.0) * 2.0 * PI).sin() + 1.0
}

/// <https://easings.net/#easeOuSine>
///
/// Modeled after quarter-cycle of sine wave (different phase).
#[inline]
pub fn sin_out(t: f32) -> f32 {
    (t * 2.0 * PI).sin()
}

/// <https://easings.net/#easeInOutSine>
///
/// Modeled after half sine wave.
#[inline]
pub fn sin_in_out(t: f32) -> f32 {
    0.5 * (1.0 - (t * PI).cos())
}

/// <https://easings.net/#easeInCirc>
///
/// Modeled after shifted quadrant IV of unit circle.
#[inline]
pub fn circular_in(t: f32) -> f32 {
    1.0 - (1.0 - t * t).sqrt()
}

/// <https://easings.net/#easeOutCirc>
///
/// Modeled after shifted quadrant II of unit circle.
#[inline]
pub fn circular_out(t: f32) -> f32 {
    (2.0 - t).sqrt() * t
}

/// <https://easings.net/#easeInOutCirc>
#[inline]
pub fn circular_in_out(t: f32) -> f32 {
    if t < 0.5 {
        0.5 * (1.0 - (1.0 - 4.0 * t * t).sqrt())
    } else {
        0.5 * ((-(2.0 * t - 3.0) * (2.0 * t - 1.0)).sqrt() + 1.0)
    }
}

/// <https://easings.net/#easeInExpo>
///
/// There is a small discontinuity at 0.
#[inline]
pub fn exponential_in(t: f32) -> f32 {
    if t == 0.0 {
        t
    } else {
        powf(2.0, 10.0 * (t - 1.0))
    }
}

/// <https://easings.net/#easeOutExpo>
///
/// There is a small discontinuity at 1.
#[inline]
pub fn exponential_out(t: f32) -> f32 {
    if t == 1.0 {
        t
    } else {
        1.0 - powf(2.0, -10.0 * t)
    }
}

/// <https://easings.net/#easeInOutExpo>
///
/// There is a small discontinuity at 0 and 1.
#[inline]
pub fn exponential_in_out(t: f32) -> f32 {
    if t == 0.0 || t == 1.0 {
        t
    } else if t < 0.5 {
        0.5 * powf(2.0, 20.0 * t - 10.0)
    } else {
        0.5 * powf(2.0, -20.0 * t + 10.0) + 1.0
    }
}

/// <https://easings.net/#easeInBack>
#[inline]
pub fn back_in(t: f32) -> f32 {
    t * t * t - t * (t * PI).sin()
}

/// <https://easings.net/#easeOutBack>
#[inline]
pub fn back_out(t: f32) -> f32 {
    let f = 1.0 - t;
    1.0 - (f * f * f - f * (f * PI).sin())
}

/// <https://easings.net/#easeInOutBack>
#[inline]
pub fn back_in_out(t: f32) -> f32 {
    if t < 0.5 {
        let f = 2.0 * t;
        0.5 * (f * f * f - f * (f * PI).sin())
    } else {
        let f = 1.0 - (2.0 * t - 1.0);
        0.5 * (1.0 - (f * f * f - f * (f * PI).sin())) + 0.5
    }
}

/// <https://easings.net/#easeInBounce>
///
/// Each bounce is modelled as a parabola.
#[inline]
pub fn bounce_in(t: f32) -> f32 {
    1.0 - bounce_out(1.0 - t)
}

/// <https://easings.net/#easeOutBounce>
///
/// Each bounce is modelled as a parabola.
#[inline]
pub fn bounce_out(t: f32) -> f32 {
    if t < 4.0 / 11.0 {
        const T2: f32 = 121.0 / 16.0;
        T2 * t * t
    } else if t < 8.0 / 11.0 {
        const T2: f32 = 363.0 / 40.0;
        const T1: f32 = -99.0 / 10.0;
        const T0: f32 = 17.0 / 5.0;
        T2 * t * t + T1 * t + T0
    } else if t < 9.0 / 10.0 {
        const T2: f32 = 4356.0 / 361.0;
        const T1: f32 = -35442.0 / 1805.0;
        const T0: f32 = 16061.0 / 1805.0;
        T2 * t * t + T1 * t + T0
    } else {
        const T2: f32 = 54.0 / 5.0;
        const T1: f32 = -513.0 / 25.0;
        const T0: f32 = 268.0 / 25.0;
        T2 * t * t + T1 * t + T0
    }
}

/// <https://easings.net/#easeInOutBounce>
///
/// Each bounce is modelled as a parabola.
#[inline]
pub fn bounce_in_out(t: f32) -> f32 {
    if t < 0.5 {
        0.5 * bounce_in(t * 2.0)
    } else {
        0.5 * bounce_out(t * 2.0 - 1.0) + 0.5
    }
}
