import { Button, Input, Modal, Text } from '@nextui-org/react';
import model from './model';
import { useSnapshot } from 'valtio';
import { getNow } from '../../utils';
import { useState } from 'react';

function genNewTask() {
  return {
    url: 'https://cdn7.caoliqi.com:65/20220223/O8AvdeJx/index.m3u8',

    output: `${Date.now()}.mp4`,
  };
}

const CreateTask: React.FC = () => {
  const state = useSnapshot(model);

  const [message, setMessage] = useState('');

  const onClose = () => (model.newTask = null);

  return (
    <>
      <Button shadow auto onClick={() => (model.newTask = genNewTask())}>
        新建下载
      </Button>

      <Modal
        closeButton
        aria-labelledby="modal-title"
        open={!!state.newTask}
        onClose={onClose}
      >
        <Modal.Header>
          <Text id="modal-title" size={18}>
            新建m3u8下载任务
          </Text>
        </Modal.Header>
        <Modal.Body>
          <Text>资源地址</Text>
          <Input
            clearable
            bordered
            fullWidth
            aria-label="资源地址"
            color="primary"
            size="lg"
            value={state.newTask?.url}
            placeholder="资源地址"
            onChange={e => (model.newTask!.url = e.target.value)}
          />
          <Text>文件保存为</Text>
          <Input
            clearable
            bordered
            fullWidth
            aria-label="输出文件"
            value={state.newTask?.output}
            color="primary"
            size="lg"
            placeholder="文件保存为"
            onChange={e => (model.newTask!.output = e.target.value)}
          />
        </Modal.Body>
        <Modal.Footer>
          <Button auto flat color="error" onClick={onClose}>
            取消
          </Button>
          <Button
            auto
            disabled={!state.newTask?.url || !state.newTask?.output}
            onClick={() => {
              const { url, output } = model.newTask as Task;

              const hasExist = model.list.some(
                ({ url: _url, status }) =>
                  url === _url && typeof status === 'number' && status! < 1,
              );
              if (hasExist) {
                setMessage('已存在下载中的相同资源，请勿重复下载');
                return;
              }

              model.list = [
                {
                  url,
                  output: output || `${Date.now()}.mp4`,
                  createAt: getNow(),
                },
                ...model.list,
              ];
              onClose();
            }}
          >
            开始下载
          </Button>
        </Modal.Footer>
      </Modal>

      <Modal
        closeButton
        aria-labelledby="modal-title"
        open={!!message}
        onClose={() => setMessage('')}
      >
        <Modal.Header>
          <Text size={18} css={{ fontWeight: 'bold' }}>
            提示
          </Text>
        </Modal.Header>
        <Modal.Body>
          <Text>{message}</Text>
        </Modal.Body>
      </Modal>
    </>
  );
};

export default CreateTask;
