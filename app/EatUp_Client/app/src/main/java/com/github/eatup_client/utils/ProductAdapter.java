package com.github.eatup_client.utils;

import android.content.Context;
import android.util.Log;
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
import com.github.eatup_client.model.ProductRes;

import java.util.ArrayList;
import java.util.List;

public class ProductAdapter extends RecyclerView.Adapter<ProductAdapter.ProductViewHolder> {

    private List<Product> productList;
    private List<OrderProduct> orderProducts;
    private Context context;
    private ProductRes productRes;

    public ProductAdapter(Context context) {
        this.context = context;
        orderProducts = new ArrayList<>();
        productRes = ProductRes.getInstance();
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
        orderProducts.clear();

        for (Product product : productList) {
            OrderProduct orderProduct = getOrderProductByProduct(product);
            if (orderProduct != null) {
                orderProducts.add(orderProduct);
            } else {
                orderProduct = new OrderProduct(0, product);
                orderProducts.add(orderProduct);
            }
        }

        notifyDataSetChanged();
    }

    private OrderProduct getOrderProductByProduct(Product product) {
        for (OrderProduct orderProduct : orderProducts) {
            if (orderProduct.getProduct().equals(product)) {
                return orderProduct;
            }
        }
        return null;
    }

    public List<Product> getOrderProducts() {
        List<Product> orderProductList = new ArrayList<>();
        for (OrderProduct orderProduct : orderProducts) {
            orderProductList.add(orderProduct.getProduct());
        }
        return orderProductList;
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
        private Product product;
        private LinearLayout llQuantity;
        private LinearLayout llAddProduct;

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


            btnDecreaseQuantity.setOnClickListener(this);
            btnIncreaseQuantity.setOnClickListener(this);
            btnAddProduct.setOnClickListener(this);
        }

        public void bind(Product product) {
            this.product = product;
            productName.setText(product.getName());
            productPrice.setText("$" + product.getPrice());
            productDescription.setText(product.getDescription());
            productImage.setImageResource(R.drawable.example_salad_img);

            updateQuantityView();

            OrderProduct orderProduct = getOrderProduct(getAdapterPosition());

            int quantity = orderProduct.getQuantity();

            if (quantity > 0) {
                llQuantity.setVisibility(View.VISIBLE);
                llAddProduct.setVisibility(View.GONE);
                tvQuantityText.setText(String.valueOf(quantity));
                Log.d("ProductAdapter", "bind: " + quantity);
            } else {
                llQuantity.setVisibility(View.GONE);
                llAddProduct.setVisibility(View.VISIBLE);
                Log.d("ProductAdapter", "bind: " + quantity);
            }
        }

        private void decreaseQuantity() {
            OrderProduct orderProduct = getOrderProduct(getAdapterPosition());
            int quantity = orderProduct.getQuantity();
            if (quantity > 0) {
                orderProduct.setQuantity(quantity - 1);
                notifyItemChanged(getAdapterPosition());
                productRes.removeProduct(product);
            }
        }

        private void increaseQuantity() {
            OrderProduct orderProduct = getOrderProduct(getAdapterPosition());
            int quantity = orderProduct.getQuantity();
            orderProduct.setQuantity(quantity + 1);
            notifyItemChanged(getAdapterPosition());
            productRes.addProduct(product);
        }

        private void addProductToCart() {
            OrderProduct orderProduct = getOrderProduct(getAdapterPosition());
            int quantity = orderProduct.getQuantity();
            orderProduct.setQuantity(quantity + 1);
            notifyItemChanged(getAdapterPosition());
            productRes.addProduct(product);
        }

        private void updateQuantityView() {
            OrderProduct orderProduct = getOrderProduct(getAdapterPosition());

            if (orderProduct != null) {
                int quantity = orderProduct.getQuantity();
                if (quantity > 0) {
                    llQuantity.setVisibility(View.VISIBLE);

                    tvQuantityText.setText(String.valueOf(quantity));
                } else {
                    llQuantity.setVisibility(View.GONE);
                }
            } else {
                llQuantity.setVisibility(View.GONE);
            }
        }

        private OrderProduct getOrderProduct(int position) {
            if (position >= 0 && position < orderProducts.size()) {
                return orderProducts.get(position);
            }
            return null;
        }

        @Override
        public void onClick(View v) {
            switch (v.getId()) {
                case R.id.btnDecreaseQuantity:
                    decreaseQuantity();
                    updateQuantityView();
                    seeAllOrderProducts(); // TODO: Remove
                    break;
                case R.id.btnIncreaseQuantity:
                    increaseQuantity();
                    updateQuantityView();
                    seeAllOrderProducts(); // TODO: Remove
                    break;
                case R.id.btnAddProduct:
                    addProductToCart();
                    updateQuantityView();
                    seeAllOrderProducts(); // TODO: Remove
                    break;
            }
        }

        // TODO: Remove
        private void seeAllOrderProducts() {
            // productRes.seeAllOrderProducts();
            Log.d("ProductAdapter", "seeAllOrderProducts: " + productRes.getOrderProducts());
        }
    }
}
