package com.github.eatup_client.utils;

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
import com.squareup.picasso.Picasso;

import java.util.List;

/**
 * Adapter for displaying order products in the resume screen RecyclerView.
 */
public class ResumeAdapter extends RecyclerView.Adapter<ResumeAdapter.ViewHolder> {
    private List<OrderProduct> orderProducts;
    private final String URL = "http://159.69.216.101/";

    public ResumeAdapter(List<OrderProduct> orderProducts) {
        this.orderProducts = orderProducts;
    }

    @NonNull
    @Override
    public ViewHolder onCreateViewHolder(@NonNull ViewGroup parent, int viewType) {
        View view = LayoutInflater.from(parent.getContext()).inflate(R.layout.item_product, parent, false);
        return new ViewHolder(view);
    }

    @Override
    public void onBindViewHolder(@NonNull ViewHolder holder, int position) {
        OrderProduct orderProduct = orderProducts.get(position);
        holder.bind(orderProduct);
    }

    @Override
    public int getItemCount() {
        return orderProducts.size();
    }

    public class ViewHolder extends RecyclerView.ViewHolder implements View.OnClickListener {
        private ImageView productImage;
        private TextView productName;
        private TextView productDescription;
        private TextView productPrice;
        private TextView tvQuantityText;
        private Button btnDecreaseQuantity;
        private Button btnIncreaseQuantity;
        private LinearLayout llQuantity;
        private LinearLayout llAddProduct;

        public ViewHolder(@NonNull View itemView) {
            super(itemView);
            productImage = itemView.findViewById(R.id.ivProductImage);
            productName = itemView.findViewById(R.id.tvProductName);
            productDescription = itemView.findViewById(R.id.tvProductDescription);
            productPrice = itemView.findViewById(R.id.tvProductPrice);
            tvQuantityText = itemView.findViewById(R.id.tvQuantityText);
            btnDecreaseQuantity = itemView.findViewById(R.id.btnDecreaseQuantity);
            btnIncreaseQuantity = itemView.findViewById(R.id.btnIncreaseQuantity);
            llQuantity = itemView.findViewById(R.id.llQuantity);
            llAddProduct = itemView.findViewById(R.id.llAddProduct);

            // Set click listeners for quantity buttons
            btnIncreaseQuantity.setOnClickListener(this);
            btnDecreaseQuantity.setOnClickListener(this);
        }

        /**
         * Binds the order product data to the ViewHolder views.
         *
         * @param orderProduct The order product to bind.
         */
        public void bind(OrderProduct orderProduct) {
            productName.setText(orderProduct.getProduct().getName());
            productPrice.setText("$" + orderProduct.getProduct().getPrice());
            productDescription.setText(orderProduct.getProduct().getDescription());
            Picasso.get()
                    .load(URL + orderProduct.getProduct().getImg_id())
                    .error(R.drawable.ic_error_load)
                    .into(productImage);

            llQuantity.setVisibility(View.VISIBLE);
            llAddProduct.setVisibility(View.GONE);

            btnDecreaseQuantity.setVisibility(View.GONE);
            btnIncreaseQuantity.setVisibility(View.GONE);

            tvQuantityText.setText(String.valueOf(orderProduct.getQuantity()));
        }

        @Override
        public void onClick(View v) {
            if (v.getId() == R.id.btnIncreaseQuantity) {
                increaseQuantity();
            } else if (v.getId() == R.id.btnDecreaseQuantity) {
                decreaseQuantity();
            }
        }

        /**
         * Increases the quantity of the order product.
         */
        private void increaseQuantity() {
            int position = getAdapterPosition();
            if (position != RecyclerView.NO_POSITION) {
                OrderProduct orderProduct = orderProducts.get(position);
                int quantity = orderProduct.getQuantity() + 1;
                orderProduct.setQuantity(quantity);
                tvQuantityText.setText(String.valueOf(quantity));
            }
        }

        /**
         * Decreases the quantity of the order product.
         * Removes the order product if the quantity becomes zero.
         */
        private void decreaseQuantity() {
            int position = getAdapterPosition();
            if (position != RecyclerView.NO_POSITION) {
                OrderProduct orderProduct = orderProducts.get(position);
                int quantity = orderProduct.getQuantity() - 1;
                if (quantity > 0) {
                    orderProduct.setQuantity(quantity);
                    tvQuantityText.setText(String.valueOf(quantity));
                } else {
                    orderProducts.remove(position);
                    notifyItemRemoved(position);
                    notifyItemRangeChanged(position, orderProducts.size());
                }
            }
        }
    }
}
