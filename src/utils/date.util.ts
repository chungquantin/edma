import { endOfDay, startOfDay } from 'date-fns';
import moment from 'moment';

export const delay = (miliseconds: number) => {
  return setTimeout(() => {
    return;
  }, miliseconds);
};

export const getStartOfDayUnix = (date: Date) => {
  return moment(startOfDay(date)).unix();
};

export const getEndOfDayUnix = (date: Date) => {
  return moment(endOfDay(date)).unix();
};

export const getStartOfTodayUnix = () => {
  return getStartOfDayUnix(new Date());
};

export const getEndOfTodayUnix = () => {
  return getEndOfDayUnix(new Date());
};

export const dateToUnix = (date: Date) => {
  return moment(date).unix();
};

export const unixToDate = (timestamp: number) => {
  return moment.unix(timestamp).toDate();
};

export const getUnixHours = (unixTime: number) => unixToDate(unixTime).getHours();

export const getDateHours = (date: Date) => date.getHours();

export const hourToSecond = (hours: number) => hours * 60 * 60;

export const getDateHoursTimestamp = (date: Date, hours: number | undefined) => {
  return getStartOfDayUnix(date) + hourToSecond(hours || 0);
};

export const getCurrentHoursTimestamp = (hours: number | undefined) => {
  return getStartOfTodayUnix() + hourToSecond(hours || 0);
};

export const getLastHourDateRange = (date: Date, from: number, to: number): [Date, Date] => {
  return [
    unixToDate(getDateHoursTimestamp(date, getDateHours(date) + from)),
    unixToDate(getDateHoursTimestamp(date, getDateHours(date) + to)),
  ];
};

export function convertDecimalToTime(decimalHour: number) {
  // Extract the whole number part (hours) and decimal part (minutes)
  const hours = Math.floor(decimalHour);
  const minutes = Math.round((decimalHour - hours) * 60);

  // Format the minutes to have leading zeros if necessary
  const formattedMinutes = minutes < 10 ? `0${minutes}` : `${minutes}`;

  // Concatenate the hours and minutes to form the final time format
  const timeString = `${hours}:${formattedMinutes}`;

  return timeString;
}

export const MONTH_LIST = [
  'Jan',
  'Feb',
  'Mar',
  'Apr',
  'May',
  'Jun',
  'Jul',
  'Aug',
  'Sep',
  'Oct',
  'Nov',
  'Dec',
];

export function getAllDatesOfMonth(year: number, month: number): Date[] {
  const date = new Date(year, month, 1);
  const dates: Date[] = [];
  let i = 0;
  while (date.getMonth() === month) {
    dates.push(new Date(date));
    date.setDate(date.getDate() + 1);
    i = i + 1;
  }
  return dates;
}
