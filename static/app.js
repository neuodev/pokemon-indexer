const content = document.getElementById("content");

main();

async function main() {
  try {
    const result = fetch("http://localhost:8080/api/v1/pokemon").then((res) =>
      res.json()
    );
    console.log(result);
  } catch (error) {
    console.log(error);
  }
}
