package com.github.eatup_client.api;

import android.content.Context;
import android.util.Log;

import androidx.lifecycle.LiveData;
import androidx.lifecycle.MutableLiveData;

import com.github.eatup_client.model.Order;
import com.github.eatup_client.model.Product;
import com.github.eatup_client.model.Session;
import com.github.eatup_client.model.SessionId;
import com.google.gson.Gson;

import java.io.IOException;
import java.util.List;

import okhttp3.Cache;
import okhttp3.OkHttpClient;
import okhttp3.ResponseBody;
import retrofit2.Call;
import retrofit2.Callback;
import retrofit2.Response;
import retrofit2.Retrofit;
import retrofit2.converter.gson.GsonConverterFactory;
import retrofit2.http.Body;
import retrofit2.http.GET;
import retrofit2.http.Header;
import retrofit2.http.Headers;
import retrofit2.http.POST;
import retrofit2.http.Path;
import retrofit2.http.Query;

public class ProductApiService {

    private static final String BASE_URL = "http://159.69.216.101/api/v1/";
    private static final String ENDPOINT_PRODUCTS = "products";
    private static final String ENDPOINT_SESSIONS = "sessions";
    private static final String ENDPOINT_SESSIONS_ID = "session_id";
    private static String userUUID = "1234567890";
    private static String TAG = "ProductApiService";

    private static ProductApiService sInstance;
    private final ProductService mProductService;
    private final SessionService mSessionService;
    private final SessionIdService mSessionIdService;
    private Context mContext;

    private ProductApiService(Context context) {
        mContext = context;
        OkHttpClient client = new OkHttpClient.Builder()
                .cache(new Cache(mContext.getCacheDir(), 10 * 1024 * 1024)) // 10 MB cache
                .build();
        Retrofit retrofit = new Retrofit.Builder()
                .baseUrl(BASE_URL)
                .client(client)
                .addConverterFactory(GsonConverterFactory.create())
                .build();

        mProductService = retrofit.create(ProductService.class);
        mSessionService = retrofit.create(SessionService.class);
        mSessionIdService = retrofit.create(SessionIdService.class);
    }

    public static synchronized ProductApiService getInstance(Context context) {
        if (sInstance == null) {
            sInstance = new ProductApiService(context.getApplicationContext());
        }
        return sInstance;
    }

    public LiveData<List<Product>> getProductsByCategory(String category) {
        MutableLiveData<List<Product>> data = new MutableLiveData<>();
        Call<List<Product>> call = mProductService.getProductsByCategory(category);
        Log.i(TAG, "getProductsByCategory: " + call.request().url());
        call.enqueue(new Callback<List<Product>>() {
            @Override
            public void onResponse(Call<List<Product>> call, Response<List<Product>> response) {
                if (response.isSuccessful()) {
                    data.setValue(response.body());
                    Log.i(TAG, "getProductsByCategory success: " + new Gson().toJson(response.body()));
                } else {
                    Log.e(TAG, "getProductsByCategory error: " + response.code());
                }
            }

            @Override
            public void onFailure(Call<List<Product>> call, Throwable t) {
                Log.e(TAG, "getProductsByCategory failure: " + t.getMessage());
            }
        });
        return data;
    }

    public LiveData<Boolean> isQRValid(String qrCode) {
        MutableLiveData<Boolean> data = new MutableLiveData<>();

        loadSessions().observeForever(sessionList -> {
            if (sessionList != null) {
                for (Session session : sessionList) {
                    Log.d(TAG, "Session ID: " + session.getId());
                    if (session.getId().equals(qrCode)) {
                        userUUID = session.getId();
                        Log.d(TAG, "Session ID matches QR code");
                        data.setValue(true);
                        return;
                    }
                }
            }
            Log.d(TAG, "Session ID does not match QR code");
            data.setValue(false);
        });

        return data;
    }

    public LiveData<List<Session>> loadSessions() {
        MutableLiveData<List<Session>> data = new MutableLiveData<>();
        Call<List<Session>> call = mSessionService.getSessions();
        call.enqueue(new Callback<List<Session>>() {
            @Override
            public void onResponse(Call<List<Session>> call, Response<List<Session>> response) {
                if (response.isSuccessful()) {
                    data.setValue(response.body());
                    Log.i(TAG, "loadSessions success: " + new Gson().toJson(response.body()));
                } else {
                    Log.e(TAG, "loadSessions error: " + response.code());
                }
            }

            @Override
            public void onFailure(Call<List<Session>> call, Throwable t) {
                Log.e(TAG, "loadSessions failure: " + t.getMessage());
            }
        });

        return data;
    }

    public LiveData<SessionId> getValidSession(String sessionId) {
        MutableLiveData<SessionId> data = new MutableLiveData<>();
        Call<SessionId> call = mSessionIdService.getSessionId(sessionId);
        Log.i(TAG, "getValidSession: " + call.request().url());
        call.enqueue(new Callback<SessionId>() {
            @Override
            public void onResponse(Call<SessionId> call, Response<SessionId> response) {
                if (response.isSuccessful()) {
                    SessionId sessionId = response.body();
                    userUUID = sessionId.getId();
                    Log.i(TAG, "getValidSession success: " + new Gson().toJson(response.body()));
                    data.setValue(sessionId);
                } else {
                    data.setValue(null);
                    Log.e(TAG, "getValidSession error: " + response.code());
                }
            }

            @Override
            public void onFailure(Call<SessionId> call, Throwable t) {
                data.setValue(null);
                Log.e(TAG, "getValidSession failure: " + t.getMessage());
            }
        });

        return data;
    }

    public void submitOrder(Order order) {
        Log.i(TAG, "submitOrder: " + new Gson().toJson(order));

        Call<ResponseBody> call = mProductService.submitOrder(userUUID, order);
        Log.i(TAG, "submitOrder: " + call.request().url());
        call.enqueue(new Callback<ResponseBody>() {
            @Override
            public void onResponse(Call<ResponseBody> call, Response<ResponseBody> response) {
                if (response.isSuccessful()) {
                    Log.i(TAG, "submitOrder success: " + new Gson().toJson(response.body()));
                    // Handle successful response
                } else {
                    Log.e(TAG, "submitOrder error: " + response.code());
                    try {
                        Log.e(TAG, "submitOrder error: " + response.errorBody().string());
                    } catch (IOException e) {
                        e.printStackTrace();
                    }
                    // Handle error response
                }
            }

            @Override
            public void onFailure(Call<ResponseBody> call, Throwable t) {
                Log.e(TAG, "submitOrder failure: " + t.getMessage());
                // Handle failure
            }
        });
    }

    public interface ProductService {
        @Headers({"device: android"})
        @GET(ENDPOINT_PRODUCTS)
        Call<List<Product>> getProductsByCategory(@Query("category") String category);

        @POST("orders/{userUUID}")
        Call<ResponseBody> submitOrder(
                @Path("userUUID") String userUUID,
                @Body Order order
        );
    }

    public interface SessionService {
        @Headers({"device: android"})
        @GET(ENDPOINT_SESSIONS)
        Call<List<Session>> getSessions();
    }

    public interface SessionIdService {
        @Headers({"device: android"})
        @GET(ENDPOINT_SESSIONS_ID + "/{session_id}")
        Call<SessionId> getSessionId(@Path("session_id") String session_id);
    }
}
