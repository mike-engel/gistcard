export const spacing = (multiple: number = 1) => multiple * 8;

export const maxWidth = spacing(90);

export enum Breakpoints {
  Mobile = 580,
  Tablet = 768,
  Desktop = 1024
}
