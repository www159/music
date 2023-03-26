import { invoke } from "@tauri-apps/api";

enum ConfigInvoke {
  getBaseConfig = "get_base_config",
  updateBaseConfig = "update_base_config",
}

export const getBaseConfig = async () => {
  return invoke<IBaseConfig>(ConfigInvoke.getBaseConfig);
};

export const updateBaseConfig = async (payload: IBaseConfig) => {
  return invoke<void>(ConfigInvoke.updateBaseConfig, { payload });
};