export interface Event {
  id: number;
  summary: string;
  description: string;
  calendar_detail_id: string,
  latitude: number;
  longtitude: number;
  location: string;
  startAt: Date;
  endAt: Date;
  all_day: boolean;
}
