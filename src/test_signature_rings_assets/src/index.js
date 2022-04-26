import { test_signature_rings } from "../../declarations/test_signature_rings";

document.querySelector("form").addEventListener("submit", async (e) => {
  e.preventDefault();
  const button = e.target.querySelector("button");

  const name = document.getElementById("name").value.toString();

  button.setAttribute("disabled", true);

  // Interact with foo actor, calling the greet method
  const greeting = await test_signature_rings.greet(name);

  button.removeAttribute("disabled");

  document.getElementById("greeting").innerText = greeting;

  return false;
});
