(function () {
  const ACCENTS = [
    { label: "Rosewater", value: "rosewater", hex: "#f5e0dc" },
    { label: "Flamingo", value: "flamingo", hex: "#f2cdcd" },
    { label: "Pink", value: "pink", hex: "#f5c2e7" },
    { label: "Mauve", value: "mauve", hex: "#cba6f7" },
    { label: "Red", value: "red", hex: "#f38ba8" },
    { label: "Maroon", value: "maroon", hex: "#eba0ac" },
    { label: "Peach", value: "peach", hex: "#fab387" },
    { label: "Yellow", value: "yellow", hex: "#f9e2af" },
    { label: "Green", value: "green", hex: "#a6e3a1" },
    { label: "Teal", value: "teal", hex: "#94e2d5" },
    { label: "Sky", value: "sky", hex: "#89dceb" },
    { label: "Sapphire", value: "sapphire", hex: "#74c7ec" },
    { label: "Blue", value: "blue", hex: "#89b4fa" },
    { label: "Lavender", value: "lavender", hex: "#b4befe" }
  ];

  const root = document.documentElement;

  function currentAccent() {
    return root.getAttribute("data-accent") || "mauve";
  }

  function setAccent(value) {
    root.setAttribute("data-accent", value);
    try {
      localStorage.setItem("notes-supply-accent", value);
    } catch (_error) {}
    window.dispatchEvent(new CustomEvent("notes-supply:accentchange"));
  }

  window.notesSupplyUi = {
    accents: ACCENTS.slice(),
    currentAccent: currentAccent,
    setAccent: setAccent
  };
})();

