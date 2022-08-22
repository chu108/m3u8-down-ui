import { Button, Input, Modal, Text } from '@nextui-org/react';
import model, { DEFAULT_SETTING } from './model';
import { useSnapshot } from 'valtio';

const GlobalConfig: React.FC = () => {
  const state = useSnapshot(model);

  const onClose = () => (model.activitySettings = null);

  return (
    <>
      <Button
        shadow
        color="secondary"
        auto
        onClick={() => (model.activitySettings = { ...model.settings })}
      >
        下载设置
      </Button>
      {state.activitySettings && (
        <Modal
          closeButton
          aria-labelledby="modal-title"
          open={!!state.activitySettings}
          onClose={onClose}
        >
          <Modal.Header>
            <Text id="modal-title" size={18}>
              配置下载参数
            </Text>
          </Modal.Header>
          <Modal.Body>
            <Text>线程数</Text>
            <Input
              clearable
              bordered
              fullWidth
              type="number"
              aria-label="线程数"
              color="primary"
              size="lg"
              value={state.activitySettings!.thread}
              placeholder="默认线程数为5"
              onChange={e =>
                (model.activitySettings!.thread = e.target.value
                  ? Number(e.target.value)
                  : undefined)
              }
            />
            <Text>代理地址</Text>
            <Input
              clearable
              bordered
              fullWidth
              aria-label="代理地址"
              value={state.activitySettings!.proxyAddress}
              color="primary"
              size="lg"
              placeholder="请设置代理地址"
              onChange={e =>
                (model.activitySettings!.proxyAddress = e.target.value)
              }
            />
          </Modal.Body>
          <Modal.Footer>
            <Button
              auto
              flat
              color="secondary"
              onClick={() => {
                model.activitySettings = { ...DEFAULT_SETTING };
              }}
            >
              重置
            </Button>
            <Button
              auto
              onClick={() => {
                const { thread = 5, proxyAddress = '' } =
                  model.activitySettings as GlobalConfig;

                model.settings = { thread, proxyAddress };
                onClose();
              }}
            >
              保存
            </Button>
          </Modal.Footer>
        </Modal>
      )}
    </>
  );
};

export default GlobalConfig;
