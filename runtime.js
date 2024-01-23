import jsonEvent from "./deploy/algo_event.json";

export function createRuntime() {
  function selfEvent() {
    // implement your runtime if needed
    return jsonEvent;
  }

  window.selfEvent = selfEvent;
}
