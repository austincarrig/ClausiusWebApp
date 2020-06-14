use super::axis::*;

pub enum ChartType
{
    Ts,
    Ph,
    Pv
}

pub enum DisplayPosition
{
    Left,
    Right
}

pub enum SubstanceType
{
    Water
}

pub struct Chart
{
    pub chart_type: ChartType,
    pub display_position: DisplayPosition,
    pub substance_type: SubstanceType,
    pub x_axis: Axis,
    pub y_axis: Axis
}

impl Chart
{
    pub fn new(chart_type_in: ChartType) -> Chart
    {
        match chart_type_in
        {
            ChartType::Ts => {
                let x_axis = Axis
                {
                    min: 0.0,
                    max: 9.3,
                    direction: Direction::X,
                    value_type: ValueType::Entropy,
                    scale_type: ScaleType::Linear
                };

                let y_axis = Axis
                {
                    min: 0.0,
                    max: 700.0,
                    direction: Direction::Y,
                    value_type: ValueType::Entropy,
                    scale_type: ScaleType::Linear
                };

                Chart
                {
                    chart_type: chart_type_in,
                    display_position: DisplayPosition::Left,
                    substance_type: SubstanceType::Water,
                    x_axis: x_axis,
                    y_axis: y_axis
                }
            }
            ChartType::Ph | ChartType::Pv => {
                panic!("oh no")
            }
        }
    }
}