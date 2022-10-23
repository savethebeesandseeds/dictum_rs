use plotly::{common::{color::Rgb, Title}, Plot, Layout};

use crate::transformer;
use crate::mathematics;

pub fn plot_example() {
  // let data_y = Vec::from([1,2,3,4,5]);
  let data_y_1 = transformer::transform_sentence(&"love".to_string());
  let data_y_2 = transformer::transform_sentence(&"fear".to_string());

  println!("data_y_1.len(): {:?}",data_y_1.len());
  println!("data_y_2.len(): {:?}",data_y_2.len());

  println!("mathematics::vector_minkowski_distance[1]: {:?}",mathematics::vector_minkowski_distance(&data_y_1,&data_y_2,1.0f32));
  println!("mathematics::vector_minkowski_distance[2]: {:?}",mathematics::vector_minkowski_distance(&data_y_1,&data_y_2,2.0f32));
  println!("mathematics::vector_minkowski_distance[3]: {:?}",mathematics::vector_minkowski_distance(&data_y_1,&data_y_2,3.0f32));
  println!("mathematics::vector_minkowski_distance[5]: {:?}",mathematics::vector_minkowski_distance(&data_y_1,&data_y_2,5.0f32));
  println!("mathematics::vector_cosine_distance: {:?}",mathematics::vector_cosine_distance(&data_y_1,&data_y_2));

  let data_x_1 = (0..data_y_1.len()).collect::<Vec<_>>();
  let data_x_2 = (0..data_y_2.len()).collect::<Vec<_>>();

  let mut plot = Plot::new();
  let layout = Layout::new()
    .plot_background_color(Rgb::new(255,255,255))
    .paper_background_color(Rgb::new(255,255,255))
    .title(Title::new("Love / Fear"));
  let trace_1 = plotly::Scatter::new(data_x_1,data_y_1)
    .fill_color(Rgb::new(0,0,0))
    .mode(plotly::common::Mode::Lines).text("Love").name("Love");
  let trace_2 = plotly::Scatter::new(data_x_2,data_y_2)
    .fill_color(Rgb::new(0,0,0))
    .mode(plotly::common::Mode::Lines).text("Fear").name("Fear");
  plot.add_trace(trace_1);
  plot.add_trace(trace_2);
  plot.set_layout(layout);
  plot.show();
}