//!/usr/bin/env node

// const ip = "http://localhost:80"
// const ip = "http://172.17.0.5:80"
const ip = "http://159.69.216.101:80"
const products = `${ip}/api/v1/products`
const orders = `${ip}/api/v1/orders`

const getRandomOrder = (products) => {
  const elements = Math.floor(Math.random() * 6) + 3;
  const order = [];
  for (let i = 0; i < elements; i++) {
    const product = products[Math.floor(Math.random() * products.length)];
    order.push({
      quantity: Math.floor(Math.random() * 2) + 1,
      product: product,
    });
  }
  return {
    products: order,
  }
}

session_id = "56d3e2a4-4d1e-4f9c-8c66-5f7abe87e67e".trim();
console.log(`session_id: ${session_id}`);

fetch(products, {
  method: 'GET',
  headers: {
    'Content-Type': 'application/json',
  },
}).then(response => response.json())
.then(products => getRandomOrder(products))
.then(data => {
  console.log('Making order with ', data.products.length, ' elements');
  fetch(orders + `/${session_id}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(data),
  }).then(response => {
    if (response.status == 200) {
      return response.json();
    }
    else {
      return response.text().then(text => {
        throw new Error(text);
      });
    }
  })
  .then(data => {
    console.log('Success:', data);
  })
  .catch((error) => {
    console.error(error);
  });
});