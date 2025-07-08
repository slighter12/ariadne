declare global {
  const chrome: {
    runtime: {
      openOptionsPage(): void;
      onInstalled: {
        addListener(callback: (details: { reason: string }) => void): void;
      };
      onMessage: {
        addListener(callback: (request: any, sender: any, sendResponse: (response: any) => void) => boolean | void): void;
      };
    };
    tabs: {
      create(createProperties: { url: string }): void;
      query(queryInfo: { active: boolean; currentWindow: boolean }, callback: (tabs: Array<{ url?: string }>) => void): void;
      onUpdated: {
        addListener(callback: (tabId: number, changeInfo: { status: string }, tab: { url?: string }) => void): void;
      };
    };
    action: {
      onClicked: {
        addListener(callback: (tab: { url?: string }) => void): void;
      };
    };
    storage: {
      local: {
        set(items: Record<string, any>): void;
      };
    };
  };
}

export {}; 