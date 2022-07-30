import dayjs from "dayjs";

export function getNow() {
  return dayjs().format("YYYY-MM-DD HH:mm:ss");
}
