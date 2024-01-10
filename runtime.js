export function createRuntime() {
  function selfEvent() {
    // implement your runtime if needed
    return {};
  }

  window.selfEvent = selfEvent;
}
