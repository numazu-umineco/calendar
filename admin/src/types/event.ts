export interface Event {
  id: number;
  summary: string;
  description: string;
  calendar_detail_id: string,
  latitude: number;
  longtitude: number;
  location: string;
  start_at: string;
  end_at: string;
  all_day: boolean;
}
