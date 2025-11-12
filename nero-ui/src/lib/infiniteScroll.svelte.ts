export function createInfiniteScroll(
  callback: () => void,
  options: {
    rootMargin?: string;
    threshold?: number | number[];
  } = {},
) {
  const { rootMargin = "100px", threshold = 0 } = options;
  let element = $state<HTMLElement>();

  $effect(() => {
    if (!element) return;
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry?.isIntersecting) {
          callback();
        }
      },
      { rootMargin, threshold },
    );
    observer.observe(element);
    return () => observer.disconnect();
  });

  return {
    get element() {
      return element;
    },
    set element(value) {
      element = value;
    },
  };
}
