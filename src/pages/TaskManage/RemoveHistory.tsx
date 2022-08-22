import { Button, Grid, Modal, Popover, Row, Text } from '@nextui-org/react';
import { useState } from 'react';
import model from './model';

const RemoveHistory: React.FC = () => {
  const [visible, setVisible] = useState(false);
  const onClose = () => setVisible(false);
  return (
    <>
      <Button shadow auto color="error" onClick={() => setVisible(true)}>
        清空历史
      </Button>
      <Modal
        closeButton
        aria-labelledby="modal-title"
        open={visible}
        onClose={onClose}
      >
        <Modal.Header>
          <Text size={18} css={{ color: '$warning' }}>
            警告
          </Text>
        </Modal.Header>
        <Modal.Body>
          <Text>
            该操作将中断进行中的下载任务并删除全部记录，请确认是否继续？
          </Text>
          <Text size={14}>(已下载完成的文件仍将继续保留)</Text>
        </Modal.Body>
        <Modal.Footer>
          <Button size="sm" light onClick={onClose}>
            取消
          </Button>
          <Button
            size="sm"
            shadow
            color="error"
            onClick={() => {
              model.list = [];
              onClose();
            }}
          >
            确定
          </Button>
        </Modal.Footer>
      </Modal>
    </>
  );
};

export default RemoveHistory;
