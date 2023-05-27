package com.github.eatup_client.utils;

import android.content.Context;
import android.util.Log;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.Button;
import android.widget.ImageView;
import android.widget.TextView;

import androidx.annotation.NonNull;
import androidx.recyclerview.widget.RecyclerView;

import com.github.eatup_client.R;
import com.github.eatup_client.api.ProductApiService;
import com.github.eatup_client.model.Order;
import com.github.eatup_client.model.OrderItem;
import com.github.eatup_client.model.Product;

import java.util.ArrayList;
import java.util.List;

public class ProductAdapter extends RecyclerView.Adapter<ProductAdapter.ProductViewHolder> {

    private List<Product> productList;
    private Context context;
    private ProductApiService productApiService;

    public ProductAdapter(Context context) {
        this.context = context;
        productApiService = new ProductApiService(context);
    }

    @NonNull
    @Override
    public ProductViewHolder onCreateViewHolder(@NonNull ViewGroup parent, int viewType) {
        View view = LayoutInflater.from(parent.getContext()).inflate(R.layout.item_product, parent, false);
        return new ProductViewHolder(view);
    }

    @Override
    public void onBindViewHolder(@NonNull ProductViewHolder holder, int position) {
        Product product = productList.get(position);
        holder.bind(product);
    }

    @Override
    public int getItemCount() {
        return productList == null ? 0 : productList.size();
    }

    public void setProducts(List<Product> productList) {
        this.productList = productList;
        notifyDataSetChanged();
    }

    public class ProductViewHolder extends RecyclerView.ViewHolder implements View.OnClickListener {

        private ImageView productImage;
        private TextView productName;
        private TextView productDescription;
        private TextView productPrice;
        private Button buyButton;
        private Product product;

        public ProductViewHolder(@NonNull View itemView) {
            super(itemView);
            productImage = itemView.findViewById(R.id.ivProductImage);
            productName = itemView.findViewById(R.id.tvProductName);
            productDescription = itemView.findViewById(R.id.tvProductDescription);
            productPrice = itemView.findViewById(R.id.tvProductPrice);
            buyButton = itemView.findViewById(R.id.btnAddProduct);
            buyButton.setOnClickListener(this);
        }

        public void bind(Product product) {
            this.product = product;
            productName.setText(product.getName());
            productPrice.setText("$" + product.getPrice());
            productDescription.setText(product.getDescription());
            productImage.setImageResource(R.drawable.example_salad_img);
        }

        @Override
        public void onClick(View v) {
            if (v.getId() == R.id.btnAddProduct) {
                Log.i("ProductAdapter", "Buy button clicked for product: " + product.getName());

                // Create an OrderItem with the selected product and quantity
                OrderItem orderItem = new OrderItem(3, product); // Pending: get the quantity from the UI

                // Create a list to hold the OrderItems
                List<OrderItem> orderItems = new ArrayList<>();
                orderItems.add(orderItem);

                // Create an Order object with the list of OrderItems
                Order order = new Order(orderItems);

                // Call the API to submit the order
                productApiService.submitOrder(order);
            }
        }
    }
}