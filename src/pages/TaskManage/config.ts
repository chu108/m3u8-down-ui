import { getNow } from '../../utils';

export enum TaskStatus {
  successful = 1,
}

export const columns = [
  {
    key: 'createAt',
    label: '创建时间',
    width: 180,
  },
  {
    key: 'output',
    label: '文件',
  },
  {
    key: 'status',
    label: '状态',
    width: 180,
  },
];

const now = getNow();
export const rows = [
  // {
  //   createAt: now,
  //   url: "http://1257120875.vod2.myqcloud.com/0ef121cdvodtransgzp1257120875/3055695e5285890780828799271/v.f230.m3u8",
  //   output: "11.mp4",
  // },
  // {
  //   createAt: now,
  //   url: "task2",
  //   status: 87,
  // },
  {
    createAt: now,
    url: 'https://xxxx',
    output: 'demo1.mp4',
    status: TaskStatus.successful,
  },
  {
    createAt: now,
    url: 'https://yyyyy',
    output: 'demo2.mp4',
    status: '失败～～～',
  },
];
