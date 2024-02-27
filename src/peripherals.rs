use crate::pac;

// We need to export this in the hal for the drivers to use

crate::peripherals! {
    SYSTICK <= SYSTICK,

    ADC1 <= ADC1,
    ADC2 <= ADC2,
    I2C1 <= I2C1,
    I2C2 <= I2C2,
    OPA <= OPA,
    SPI1 <= SPI1,
    SPI2 <= SPI2,
    SPI3 <= SPI3,

    PIOC <= virtual,

    TIM1 <= TIM1,
    TIM3 <= TIM3,

    USART1 <= USART1,
    USART2 <= USART2,
    USART3 <= USART3,

    EXTI <= EXTI,

    EXTI0 <= virtual,
    EXTI1 <= virtual,
    EXTI2 <= virtual,
    EXTI3 <= virtual,
    EXTI4 <= virtual,
    EXTI5 <= virtual,
    EXTI6 <= virtual,
    EXTI7 <= virtual,
    EXTI8 <= virtual,
    EXTI9 <= virtual,
    EXTI10 <= virtual,
    EXTI11 <= virtual,
    EXTI12 <= virtual,
    EXTI13 <= virtual,
    EXTI14 <= virtual,
    EXTI15 <= virtual,
    EXTI16 <= virtual,
    EXTI17 <= virtual,
    EXTI18 <= virtual,
    EXTI19 <= virtual,
    EXTI20 <= virtual,
    EXTI21 <= virtual,
    EXTI22 <= virtual,
    EXTI23 <= virtual,

    PA0 <= virtual,
    PA1 <= virtual,
    PA2 <= virtual,
    PA3 <= virtual,
    PA4 <= virtual,
    PA5 <= virtual,
    PA6 <= virtual,
    PA7 <= virtual,
    PA8 <= virtual,
    PA9 <= virtual,
    PA10 <= virtual,
    PA11 <= virtual,
    PA12 <= virtual,
    PA13 <= virtual,
    PA14 <= virtual,
    PA15 <= virtual,

    PB0 <= virtual,
    PB1 <= virtual,
    PB2 <= virtual,
    PB3 <= virtual,
    PB4 <= virtual,
    PB5 <= virtual,
    PB6 <= virtual,
    PB7 <= virtual,
    PB8 <= virtual,
    PB9 <= virtual,
    PB10 <= virtual,
    PB11 <= virtual,
    PB12 <= virtual,
    PB13 <= virtual,
    PB14 <= virtual,
    PB15 <= virtual,

    PC0 <= virtual,
    PC1 <= virtual,
    PC2 <= virtual,
    PC3 <= virtual,
    PC4 <= virtual,
    PC5 <= virtual,
    PC6 <= virtual,
    PC7 <= virtual,
    PC8 <= virtual,
    PC9 <= virtual,
    PC10 <= virtual,
    PC11 <= virtual,
    PC12 <= virtual,
    PC13 <= virtual,
    PC14 <= virtual,
    PC15 <= virtual,

    PD0 <= virtual,
    PD1 <= virtual,
    PD2 <= virtual,
    PD3 <= virtual,
    PD4 <= virtual,
    PD5 <= virtual,
    PD6 <= virtual,
    PD7 <= virtual,
    PD8 <= virtual,
    PD9 <= virtual,
    PD10 <= virtual,
    PD11 <= virtual,
    PD12 <= virtual,
    PD13 <= virtual,
    PD14 <= virtual,
    PD15 <= virtual,

    PE0 <= virtual,
    PE1 <= virtual,
    PE2 <= virtual,
    PE3 <= virtual,
    PE4 <= virtual,
    PE5 <= virtual,
    PE6 <= virtual,
    PE7 <= virtual,
    PE8 <= virtual,
    PE9 <= virtual,
    PE10 <= virtual,
    PE11 <= virtual,
    PE12 <= virtual,
    PE13 <= virtual,
    PE14 <= virtual,
    PE15 <= virtual,
}
