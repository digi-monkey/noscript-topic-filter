import algo_event from "./algo_event.json";

export function createRuntime() {
  function selfEvent() {
    return algo_event;
  }

  window.selfEvent = selfEvent;
}
