const content = document.getElementById("content");

const state = {
  page: 1,
  pageSize: 12,
};

let infScroll = new InfiniteScroll("#content", {
  path: `/api/v1/pokemon?page={{#}}&page_size=12`,
  responseBody: "json",
  history: false,
});

infScroll.on("load", function (data) {
  let html = "";

  data.urls.forEach((url) => {
    html += `
        <div class="col-span-6 md:col-span-4 xl:col-span-3 bg-gray-100">
            <img
                src="${url}"
                alt="Pokemon"
                title="Pokemon"
                loading="lazy"
            />
        </div>
        `;
  });
  content.innerHTML += html;
});

infScroll.loadNextPage();
