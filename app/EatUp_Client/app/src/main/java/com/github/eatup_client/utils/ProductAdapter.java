package com.github.eatup_client.utils;

import android.content.Context;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.Button;
import android.widget.ImageView;
import android.widget.LinearLayout;
import android.widget.TextView;

import androidx.annotation.NonNull;
import androidx.recyclerview.widget.RecyclerView;

import com.github.eatup_client.R;
import com.github.eatup_client.model.OrderProduct;
import com.github.eatup_client.model.Product;
import com.github.eatup_client.model.MVCManager;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

/**
 * Adapter for displaying products in the RecyclerView with quantity and add functionality.
 */
public class ProductAdapter extends RecyclerView.Adapter<ProductAdapter.ProductViewHolder> {

    private List<Product> productList;
    private Map<Product, OrderProduct> orderProductMap;
    private MVCManager mvcManager;
    private Context context;

    public ProductAdapter(Context context) {
        this.context = context;
        orderProductMap = new HashMap<>();
        mvcManager = MVCManager.getInstance();
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
        orderProductMap.clear();

        List<OrderProduct> orderProducts = mvcManager.getOrderProducts();

        for (Product product : productList) {
            OrderProduct orderProduct = null;
            for (OrderProduct existingOrderProduct : orderProducts) {
                if (existingOrderProduct.getProduct().getName().equals(product.getName())) {
                    orderProduct = existingOrderProduct;
                    break;
                }
            }

            if (orderProduct != null) {
                orderProductMap.put(product, orderProduct);
            } else {
                orderProduct = new OrderProduct(0, product);
                orderProductMap.put(product, orderProduct);
            }
        }

        notifyDataSetChanged();
    }

    public class ProductViewHolder extends RecyclerView.ViewHolder implements View.OnClickListener {

        private ImageView productImage;
        private TextView productName;
        private TextView productDescription;
        private TextView productPrice;
        private TextView tvQuantityText;
        private Button btnDecreaseQuantity;
        private Button btnIncreaseQuantity;
        private Button btnAddProduct;
        private LinearLayout llQuantity;
        private LinearLayout llAddProduct;

        private Product product;
        private OrderProduct orderProduct;

        public ProductViewHolder(@NonNull View itemView) {
            super(itemView);
            productImage = itemView.findViewById(R.id.ivProductImage);
            productName = itemView.findViewById(R.id.tvProductName);
            productDescription = itemView.findViewById(R.id.tvProductDescription);
            productPrice = itemView.findViewById(R.id.tvProductPrice);
            tvQuantityText = itemView.findViewById(R.id.tvQuantityText);
            btnDecreaseQuantity = itemView.findViewById(R.id.btnDecreaseQuantity);
            btnIncreaseQuantity = itemView.findViewById(R.id.btnIncreaseQuantity);
            btnAddProduct = itemView.findViewById(R.id.btnAddProduct);
            llQuantity = itemView.findViewById(R.id.llQuantity);
            llAddProduct = itemView.findViewById(R.id.llAddProduct);

            // Set click listeners for quantity buttons and add button
            btnDecreaseQuantity.setOnClickListener(this);
            btnIncreaseQuantity.setOnClickListener(this);
            btnAddProduct.setOnClickListener(this);
        }

        /**
         * Binds the product data to the ViewHolder views.
         *
         * @param product The product to bind.
         */
        public void bind(Product product) {
            this.product = product;
            orderProduct = getOrderProduct(product);

            productName.setText(product.getName());
            productPrice.setText("$" + product.getPrice());
            productDescription.setText(product.getDescription());
            productImage.setImageResource(R.drawable.example_salad_img);

            updateQuantityView();
        }

        /**
         * Updates the visibility of the quantity view based on the order product's quantity.
         */
        private void updateQuantityView() {
            if (orderProduct != null && orderProduct.getQuantity() > 0) {
                llQuantity.setVisibility(View.VISIBLE);
                tvQuantityText.setText(String.valueOf(orderProduct.getQuantity()));
                llAddProduct.setVisibility(View.GONE);
            } else {
                llQuantity.setVisibility(View.GONE);
                llAddProduct.setVisibility(View.VISIBLE);
            }
        }

        /**
         * Retrieves the corresponding OrderProduct for the given Product.
         *
         * @param product The product to retrieve the OrderProduct for.
         * @return The corresponding OrderProduct or null if not found.
         */
        private OrderProduct getOrderProduct(Product product) {
            return orderProductMap.get(product);
        }

        /**
         * Updates the order product's quantity and notifies the adapter of the change.
         *
         * @param quantity The new quantity value.
         */
        private void updateOrderProduct(int quantity) {
            if (orderProduct != null) {
                mvcManager.setQuantity(orderProduct.getProduct(), quantity);
                orderProduct.setQuantity(quantity);
            } else {
                orderProduct = new OrderProduct(quantity, product);
                orderProductMap.put(product, orderProduct);
                mvcManager.addProduct(product);
            }

            notifyItemChanged(getAdapterPosition());
        }

        @Override
        public void onClick(View v) {
            int quantity = orderProduct.getQuantity();

            if (v.getId() == R.id.btnDecreaseQuantity) {
                quantity--;
            } else if (v.getId() == R.id.btnIncreaseQuantity || v.getId() == R.id.btnAddProduct) {
                quantity++;
            }

            updateOrderProduct(quantity);
            updateQuantityView();
        }

    }
}
