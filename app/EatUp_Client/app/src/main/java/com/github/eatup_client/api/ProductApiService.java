package com.github.eatup_client.api;

import com.github.eatup_client.model.Product;

import java.util.List;

import retrofit2.Call;
import retrofit2.Callback;
import retrofit2.Retrofit;
import retrofit2.converter.gson.GsonConverterFactory;
import retrofit2.http.GET;
import retrofit2.http.Query;

public class ProductApiService {

    private static final String BASE_URL = "http://49.13.5.120:8000/api/v1/";
    private static final String ENDPOINT_PRODUCTS = "products";

    private final Retrofit retrofit;

    public ProductApiService() {
        retrofit = new Retrofit.Builder()
                .baseUrl(BASE_URL)
                .addConverterFactory(GsonConverterFactory.create())
                .build();
    }

    public void getProductsByCategory(String category, Callback<List<Product>> callback) {
        ProductService productService = retrofit.create(ProductService.class);
        Call<List<Product>> call = productService.getProductsByCategory(category);
        call.enqueue(callback);
    }

    public interface ProductService {
        @GET(ENDPOINT_PRODUCTS)
        Call<List<Product>> getProductsByCategory(@Query("category") String category);
    }
}
