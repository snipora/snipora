if (import.meta.env.PROD) {
  document.documentElement.addEventListener("contextmenu", event => {
    event.preventDefault();
  });
}
