import { proxy } from 'valtio';
import { subscribeKey } from 'valtio/utils';
import { rows, TaskStatus } from './config';
import { invoke } from '@tauri-apps/api';
import { listen } from '@tauri-apps/api/event';

let initialData = {
  newTask: null as Task | null,
  list: rows as Task[],
};
try {
  const data = localStorage.getItem('TASK_STATE');
  if (data) {
    initialData = Object.assign(initialData, JSON.parse(data));
  }
} catch (error) {
  console.error(error);
}

const model = proxy(initialData);

function setTaskList(list: Task[]) {
  model.list = list;
}

subscribeKey(model, 'list', list => {
  console.log('task list ', list);
  let hasNewTask = false;
  const newList = list.map(task => {
    if (task.status === undefined) {
      hasNewTask = true;
      invoke('down_file', { filePath: task.url, output: task.output }).then(
        response => console.log(response),
      );
      task.status = 0;
    }
    return task;
  });
  if (hasNewTask) {
    setTaskList(newList);
  }
});

listen('downing', event => {
  console.log('downing event', event);
  try {
    const res = JSON.parse((event.payload as any)?.message);
    console.log('downing res', res);
    const { url, total, downloaded, err } = res;

    if (url) {
      console.log('downing url', url);
      const index = model.list.findIndex(
        e => e.url === url,
        // e.url === 'https://cdn7.caoliqi.com:65/20220223/O8AvdeJx/index.m3u8',
      );
      if (index < 0) {
        console.warn('没有对应的task, url：', url);
        return;
      }
      const task = model.list[index];

      if (err) {
        task!.status = TaskStatus.failed;
        console.error(err);
        return;
      }
      if (total && downloaded) {
        const newPercent = downloaded / total;
        console.log('newPercent', newPercent);
        task!.status = newPercent;
      }
      return;
    }
  } catch (error) {
    console.error(error);
  }
});

export default model;
