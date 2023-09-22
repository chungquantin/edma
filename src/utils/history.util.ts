/* eslint-disable no-regex-spaces */
const reLine = /^ *(\d+)  (\d+)  (\d+):(\d+)  (\S+) ?(.*)?/;

function multilineCommand(commands: string[], curr: string) {
  if (!commands.length || curr.match(reLine)) {
    commands.push(curr);
  } else {
    const last = commands.pop();
    commands.push(`${last}${curr}`);
  }

  return commands;
}

export function parseOmzHistoryLine(line: string) {
  const firstSplits = line.split(':');
  if (firstSplits.length < 3) return;
  const timestamp = parseInt(firstSplits[1].trim());
  const command = firstSplits[2].split(';')[1];
  return {
    timestamp,
    command,
  };
}

export const parseOmzHistory = (historyContent: string) => {
  return historyContent.trim().split('\n').reduce(multilineCommand, []).map(parseOmzHistoryLine);
};
