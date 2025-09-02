import type { Car, UserData, UserStub } from './models';

export function validateCar(
  user: UserData,
  departureTime: string,
  returnTime: string,
  maxCapacity: number,
  riders: UserStub[],
  otherCars: Car[]
) {
  const out = [];
  if (departureTime.length == 0 || returnTime.length == 0) {
    out.push('All times must be filled in.');
  }
  if (new Date(returnTime) < new Date(departureTime)) {
    out.push('Return time cannot be before departure.');
  }
  if (new Date(departureTime) < new Date()) {
    out.push('Car cannot leave in the past.');
  }
  if (maxCapacity < 0) {
    out.push('Capacity must be greater than or equal to 0.');
  }
  if (riders.length > maxCapacity) {
    out.push('You have too many riders for your capacity.');
  }
  if (riders.find((rider) => rider.id === user.id) != null) {
    out.push('You cannot be a rider in your own car.');
  }
  const otherCarMembers = otherCars
    .flatMap((car) => [car.driver, ...car.riders])
    .map((user) => user.id);
  riders.forEach((rider) => {
    if (otherCarMembers.includes(rider.id)) {
      out.push(`${rider.name} is already in another car or is a driver.`);
    }
  });
  return out;
}

export function validateEvent(title: string, location: string, start: string, end: string) {
  const out = [];
  if (title.length == 0 || location.length == 0 || start.length == 0 || end.length == 0) {
    out.push('Please fill in all fields.');
  }
  if (new Date(start) > new Date(end)) {
    out.push('Start date cannot be after end.');
  }
  // if (new Date(end) < new Date()) {
  //   out.push('Event cannot be in the past.');
  // }
  return out;
}
