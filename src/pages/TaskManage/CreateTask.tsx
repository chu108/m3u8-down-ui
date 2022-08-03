import { Button, Input, Modal, Text } from "@nextui-org/react";
import model from "./model";
import { useSnapshot } from "valtio";
import { getNow } from "../../utils";

const DEFAULT_TASK = { url: "http://1257120875.vod2.myqcloud.com/0ef121cdvodtransgzp1257120875/3055695e5285890780828799271/v.f230.m3u8", output: "11.MP4" };

const CreateTask: React.FC = () => {
  const state = useSnapshot(model);

  const onClose = () => (model.newTask = null);

  return (
    <>
      <Button shadow auto onClick={() => (model.newTask = DEFAULT_TASK)}>
        新建任务
      </Button>
      {state.newTask && (
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
            <Input
              clearable
              bordered
              fullWidth
              aria-label="资源地址"
              color="primary"
              size="lg"
              value={model.newTask!.url}
              placeholder="资源地址"
              onChange={(e) => (model.newTask!.url = e.target.value)}
            />
            <Input
              clearable
              bordered
              fullWidth
              aria-label="输出文件"
              value={model.newTask!.output}
              color="primary"
              size="lg"
              placeholder="文件保存为"
              onChange={(e) => (model.newTask!.output = e.target.value)}
            />
          </Modal.Body>
          <Modal.Footer>
            <Button auto flat color="error" onClick={onClose}>
              取消
            </Button>
            <Button
              auto
              disabled={!state.newTask?.url}
              onClick={() => {
                const { url, output } = model.newTask as Task;
                model.list = [
                  {
                    url,
                    output: output || "m3u8下载器-未命名文件.mp4",
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
      )}
    </>
  );
};

export default CreateTask;
