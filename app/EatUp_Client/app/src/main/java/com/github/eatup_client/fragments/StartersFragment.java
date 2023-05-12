package com.github.eatup_client.fragments;

import android.os.Bundle;

import androidx.fragment.app.Fragment;
import androidx.recyclerview.widget.LinearLayoutManager;
import androidx.recyclerview.widget.RecyclerView;

import android.util.Log;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.Toast;

import com.github.eatup_client.R;
import com.github.eatup_client.api.ProductApiService;
import com.github.eatup_client.model.Product;
import com.github.eatup_client.utils.ProductAdapter;

import java.util.ArrayList;
import java.util.List;

import retrofit2.Call;
import retrofit2.Callback;
import retrofit2.Response;

public class StartersFragment extends Fragment {

    private RecyclerView recyclerView;
    private ProductAdapter adapter;
    private List<Product> productList;

    private ProductApiService productApiService;

    @Override
    public View onCreateView(LayoutInflater inflater, ViewGroup container, Bundle savedInstanceState) {
        System.out.println("StartersFragment.onCreateView");
        View view = inflater.inflate(R.layout.fragment_starters, container, false);

        recyclerView = view.findViewById(R.id.rv_starters);
        recyclerView.setLayoutManager(new LinearLayoutManager(getActivity()));

        productList = new ArrayList<>();
        adapter = new ProductAdapter(getActivity());
        recyclerView.setAdapter(adapter);

        // Initialize ProductApiService
        productApiService = new ProductApiService();

        // Get starters products from API
        productApiService.getProductsByCategory("Starters", new Callback<List<Product>>() {
            @Override
            public void onResponse(Call<List<Product>> call, Response<List<Product>> response) {
                if (response.isSuccessful()) {
                    productList.clear();
                    productList.addAll(response.body());
                    adapter.setProducts(productList);
                    Log.i("StartersFragment", "Starters products: " + productList.get(0).getName());
                } else {
                    // Handle error
                    Log.e("StartersFragment", "Error getting starters products: " + response.code());
                }
            }

            @Override
            public void onFailure(Call<List<Product>> call, Throwable t) {
                // Handle failure
                Log.e("StartersFragment", "Error getting starters products", t);
            }
        });

        return view;
    }
}
