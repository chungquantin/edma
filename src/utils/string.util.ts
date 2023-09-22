export const breaklineCharacter = '\\n';

export const shortenString = (str: string, maxLength: number) =>
  str?.length > maxLength ? `${str.slice(0, maxLength)}...` : str;

export const stringInArray = (stringArray: string[], cmpString: string): boolean =>
  stringArray.some(ele => ele === cmpString);

export const isValidUrl = (url: string) => {
  const hrefRegex = /^(https?:\/\/)?(www\.)?[a-zA-Z0-9-]+(\.[a-zA-Z]{2,})+(\/\S*)?$/;
  return hrefRegex.test(url);
};

export function makeid(length: number) {
  let result = '';
  const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  const charactersLength = characters.length;
  let counter = 0;
  while (counter < length) {
    result += characters.charAt(Math.floor(Math.random() * charactersLength));
    counter += 1;
  }
  return result;
}

export function formatBytes(bytes: number, decimals = 2) {
  if (!+bytes) return '0 Bytes';

  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = ['Bytes', 'KiB', 'MiB', 'GiB', 'TiB', 'PiB', 'EiB', 'ZiB', 'YiB'];

  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`;
}

export function toHex(str: string) {
  let result = '';
  for (let i = 0; i < str.length; i++) {
    result += str.charCodeAt(i).toString(16);
  }
  return result;
}

export function generateHSLColor(text: string): string {
  // Calculate a numeric value based on the text
  let numericValue = 0;
  for (let i = 0; i < text.length; i++) {
    numericValue += text.charCodeAt(i);
  }

  // Generate HSL values
  const hue = numericValue % 360; // Hue value between 0 and 359
  const saturation = 70; // Fixed saturation value of 70%
  const lightness = 50; // Fixed lightness value of 50%

  // Construct the HSL color string
  const hslColor = `hsl(${hue}, ${saturation}%, ${lightness}%)`;
  return hslColor;
}

export const voidCallback = () => {
  return;
};

export function renderTime(seconds: number) {
  const _seconds = parseInt(seconds?.toString());
  const hours = Math.floor(_seconds / 3600);
  const minutes = Math.floor((_seconds % 3600) / 60);
  const remainingSeconds = _seconds % 60;

  const timeParts: string[] = [];
  if (hours > 0) {
    timeParts.push(`${hours} hour${hours !== 1 ? 's' : ''}`);
  }
  if (minutes > 0) {
    timeParts.push(`${minutes} minute${minutes !== 1 ? 's' : ''}`);
  }
  if (remainingSeconds > 0 || timeParts.length === 0) {
    timeParts.push(`${remainingSeconds} second${remainingSeconds !== 1 ? 's' : ''}`);
  }

  const timeString = timeParts.join(' ');
  return timeString;
}

export function convertSecondsToTime(seconds: number) {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const remainingSeconds = seconds % 60;

  const timeString = `${padNumber(hours)}:${padNumber(minutes)}:${padNumber(remainingSeconds)}`;
  return timeString;
}

function padNumber(number: number) {
  return String(number).padStart(2, '0');
}

export const getURLOrigin = (url: string) => {
  try {
    const _url = new URL(url);
    return _url.origin;
  } catch (error) {
    return url;
  }
};

export const getURLHost = (url: string) => {
  try {
    const _url = new URL(url);
    return _url.hostname;
  } catch (error) {
    return url;
  }
};

export const isValidEmail = (email: string) => {
  return String(email)
    .toLowerCase()
    .match(
      /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|.(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/
    );
};

export const generateRandomRgbaStr = () => {
  return `rgba(${Math.floor(Math.random() * 150) + 0}, ${Math.floor(Math.random() * 150) + 0}, ${
    Math.floor(Math.random() * 150) + 0
  })`;
};

export const formatCommandOutput = (input: string) => {
  return input.replaceAll(breaklineCharacter, '').replaceAll('"', '');
};

export const formatHistoryDirectoryName = (name: string) => {
  return name.replace('.', '').replace('_history', '');
};

export const getFileNameFromPath = (path: string) => {
  const splitted_path = path.split('/');
  return splitted_path[splitted_path.length - 1];
};
