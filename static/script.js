const endDate = new Date(2025, 5, 18, 12, 0, 0);

const getHoursRemaining = () => {
  const now = new Date();
  const diffInMilliseconds = endDate - now;
  const diffInHours = Math.floor(diffInMilliseconds / (1000 * 60 * 60));
  return diffInHours;
};

const getMinutesRemaining = () => {
  const now = new Date();
  const diffInMilliseconds = endDate - now;
  const diffInMinutes = Math.floor(diffInMilliseconds / (1000 * 60));
  return diffInMinutes;
};

const getSecondsRemaining = () => {
  const now = new Date();
  const diffInMilliseconds = endDate - now;
  const diffInSeconds = Math.floor(diffInMilliseconds / 1000);
  return diffInSeconds;
};

const addDataToDom = () => {
  const HoursRemaining = getHoursRemaining();
  console.log(HoursRemaining);
};

addDataToDom();
