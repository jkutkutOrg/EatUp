package com.github.eatup_client.fragments;

import android.os.Bundle;

import androidx.fragment.app.Fragment;
import androidx.lifecycle.LiveData;
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
    private LiveData<List<Product>> productListLiveData;

    private ProductApiService productApiService;

    @Override
    public View onCreateView(LayoutInflater inflater, ViewGroup container, Bundle savedInstanceState) {
        Log.i("Starters", "Create VIEW StartersFragment");
        View view = inflater.inflate(R.layout.fragment_starters, container, false);

        recyclerView = view.findViewById(R.id.rv_starters);
        recyclerView.setLayoutManager(new LinearLayoutManager(getActivity()));

        adapter = new ProductAdapter(getContext());
        recyclerView.setAdapter(adapter);

        productApiService = new ProductApiService(getContext());
        productListLiveData = productApiService.getProductsByCategory("Starters");
        productListLiveData.observe(getViewLifecycleOwner(), productList -> {
            adapter.setProducts(productList);
            Log.i("Starters", "Starters products: " + productList.get(0).getName());
        });

        return view;
    }
}
