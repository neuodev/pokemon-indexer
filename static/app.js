const content = document.getElementById("content");

const state = {
  page: 1,
  pageSize: 12,
};

main();

async function main() {
  content.innerHTML = `
    <div class="w-full flex items-center justify-center col-span-12">
        <p class='text-indigo-900 text-xl font-bold'>Processing...</p>
    </div>
    `;

  try {
    const result = await fetch(getUrl()).then((res) => res.json());

    console.log(result);
    let html = "";

    result.urls.forEach((url) => {
      html += `
        <div class="col-span-12 sm:col-span-6 md:col-span-4 xl:col-span-3 bg-gray-100">
            <img
                src="${url}"
                alt="Pokemon"
                title="Pokemon"
                loading="lazy"
            />
        </div>
        `;
    });

    content.innerHTML = html;
  } catch (error) {
    alert("Error getting pokemons from the server. please retry");
  }
}

function getUrl() {
  const { page, pageSize } = state;
  return `http://localhost:8080/api/v1/pokemon?page=${page}&page_size=${pageSize}`;
}
