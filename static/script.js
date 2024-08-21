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

const getFormattedTimeLeft = () => {
  const minutesRemaining = getMinutesRemaining();
  const secondsRemaining = getSecondsRemaining();

  const hoursLeft = getHoursRemaining();

  let minutesLeft = minutesRemaining % (hoursLeft * 60);
  if (minutesLeft < 10) {
    minutesLeft = `0${minutesLeft}`;
  }

  let secondsLeft = secondsRemaining % (minutesRemaining * 60);
  if (secondsLeft < 10) {
    secondsLeft = `0${secondsLeft}`;
  }

  return {
    hoursLeft,
    minutesLeft,
    secondsLeft,
  };
};

const addDataToDom = () => {
  const formattedTimeLeft = getFormattedTimeLeft();

  const hoursElement = document.querySelector("#hours-remaining");
  const minutesElement = document.querySelector("#minutes-remaining");
  const secondsElement = document.querySelector("#seconds-remaining");

  hoursElement.innerText = formattedTimeLeft.hoursLeft;
  minutesElement.innerText = formattedTimeLeft.minutesLeft;
  secondsElement.innerText = formattedTimeLeft.secondsLeft;
};

setInterval(() => {
  addDataToDom();
}, 1000);
